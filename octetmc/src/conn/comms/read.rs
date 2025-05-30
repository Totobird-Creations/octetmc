use super::ConnPeerComms;
use crate::conn::{ ConnPeerResult, ConnPeerError };
use crate::util::future::timeout;
use octetmc_protocol::value::varint::{ VarInt, VarIntDecodeError };
use octetmc_protocol::packet::decode::{ PacketPrefixedDecode, PacketPartDecode, DecodeBufHead, DecodeBuf, UnknownPrefix, MAX_PACKET_LENGTH };
use core::time::Duration;
use core::mem::{ self, MaybeUninit, ManuallyDrop };
use core::ptr;
use core::pin::Pin;
use core::ops::Deref;
use std::io::{ self, Write };
use std::borrow::Cow;
use smol::io::AsyncReadExt;
use flate2::write::ZlibDecoder;


impl ConnPeerComms {

    pub(crate) async fn read_packet<P>(&mut self) -> ConnPeerResult<ReadPacketContainer<P>>
    where
        P : PacketPrefixedDecode
    {
        let (total_len, _,) = self.read_packet_len().await?;
        self.wait_for_bytes(total_len).await?; // <NOTE 1>

        let uncompressed_packet_len;
        let compressed_packet_len;
        let is_compressed;
        let decompressed_packet_len;
        if (self.compress_threshold.is_some()) {

            let consumed;
            (uncompressed_packet_len, consumed,) = self.read_packet_len().await?;
            compressed_packet_len  = total_len - consumed;
            is_compressed          = uncompressed_packet_len != 0;
            decompressed_packet_len = if (is_compressed) {
                if (uncompressed_packet_len > compressed_packet_len) {
                    return Err(ConnPeerError::PacketTooLong);
                }
                uncompressed_packet_len
            } else { compressed_packet_len };

        } else {

            is_compressed           = false;
            compressed_packet_len   = 0;
            decompressed_packet_len = total_len;

        }

        let buf = Pin::from(if (is_compressed) {
            let buf = Box::<[u8]>::new_uninit_slice(decompressed_packet_len);

            let mut z = ZlibDecoder::new(PacketDecompress { buf, head : 0 });
            let (a, b,) = self.read_queue.as_slices();
            let consuming = compressed_packet_len.min(a.len());
            // SAFETY: `consuming` is clamped down to `a.len()` on the line above.
            if (z.write_all(unsafe { a.get_unchecked(0..consuming) }).is_err()) {
                return Err(ConnPeerError::BadPacket(Cow::Borrowed("failed to decompress")));
            }
            let consuming = compressed_packet_len - consuming;
            // SAFETY: `consuming` can never be `b.len()` or higher, as `self.read_queue` has enough bytes for the whole packet (see <NOTE 1>).
            if (z.write_all(unsafe { b.get_unchecked(0..consuming) }).is_err()) {
                return Err(ConnPeerError::BadPacket(Cow::Borrowed("failed to decompress")));
            }

            self.read_queue.drain(0..compressed_packet_len);
            // SAFETY: `<PacketDecompress as Write>::flush` can never return `Err`.
            let d = unsafe { z.finish().unwrap_unchecked() };
            if (d.head < d.buf.len()) {
                return Err(ConnPeerError::BadPacket(Cow::Borrowed("failed to decompress")));
            }
            // SAFETY: All bytes in `d.buf` were written as checked in the if-condition above.
            unsafe { d.buf.assume_init() }

        } else {
            let mut buf = Box::<[u8]>::new_uninit_slice(decompressed_packet_len);

            let (a, b,) = self.read_queue.as_slices();
            let consuming = decompressed_packet_len.min(a.len());
            // SAFETY: `consuming` is clamped down to `a.len()` on the line above.
            let a = unsafe { a.get_unchecked(0..consuming) };
            unsafe { ptr::copy_nonoverlapping(a.as_ptr(), buf.as_mut_ptr() as _, a.len()); }
            let consuming = decompressed_packet_len - consuming;
            // SAFETY: `consuming` can never be `b.len()` or higher, as `self.read_queue` has enough bytes for the whole packet (see <NOTE 1>).
            let b = unsafe { b.get_unchecked(0..consuming) };
            unsafe { ptr::copy_nonoverlapping(b.as_ptr(), buf.as_mut_ptr().byte_add(a.len()) as _, b.len()); }

            self.read_queue.drain(0..decompressed_packet_len);
            // SAFETY: All bytes in `buf` were written.
            unsafe { buf.assume_init() }
        });

        let mut head = DecodeBufHead::default();
        // SAFETY: `PacketDecodeContainer.packet` will not outlive `PacketDecodeContainer.raw`.
        //         `PacketDecodeContainer.raw` can not move because it is `Pin<Box<_>>`.
        let packet = P::decode_prefixed(DecodeBuf::from(unsafe { mem::transmute::<&[u8], &[u8]>(&*buf) }), &mut head).map_err(|e| match (e) {
            UnknownPrefix::UnknownPrefix(p) => ConnPeerError::UnknownPacketPrefix(p),
            UnknownPrefix::Error(e)         => ConnPeerError::BadPacket(e.into()),
        })?;
        if (head.consumed() < buf.len()) { return Err(ConnPeerError::NoPacketEnd); }
        Ok(ReadPacketContainer {
            raw    : ManuallyDrop::new(buf),
            packet : ManuallyDrop::new(packet)
        })
    }

    pub(crate) async fn read_packet_timeout<P>(&mut self, dur : Duration) -> ConnPeerResult<ReadPacketContainer<P>>
    where
        P : PacketPrefixedDecode
    { match (timeout(dur, self.read_packet::<P>()).await) {
        Ok(Ok(out))  => Ok(out),
        Ok(Err(err)) => Err(err),
        Err(_)       => Err(ConnPeerError::TimedOut)
    } }


    /// Returns (length value, consumed byte count,).
    async fn read_packet_len(&mut self) -> ConnPeerResult<(usize, usize,)> {
        let mut buf   = [0u8; VarInt::<u32>::MAX_BYTES];
        let mut index = 0;
        loop {
            match (self.read_queue.pop_front()) {
                None => { self.read_more_bytes().await?; },
                Some(b) => {
                    buf[index] = b;
                    index += 1;
                    match (VarInt::<u32>::decode(DecodeBuf::from(&buf[0..index]), &mut DecodeBufHead::default())) {
                        Err(VarIntDecodeError::IncompleteData) => { },
                        Err(VarIntDecodeError::TooLong) => {
                            return Err(ConnPeerError::InvalidPacketLength);
                        },
                        Ok(value) => {
                            let value = *value as usize;
                            return if (value > MAX_PACKET_LENGTH) {
                                Err(ConnPeerError::PacketTooLong)
                            } else { Ok((value, index,)) }
                        }
                    }
                    if (index >= VarInt::<u32>::MAX_BYTES) {
                        return Err(ConnPeerError::InvalidPacketLength);
                    }
                }
            }
        }
    }

    async fn wait_for_bytes(&mut self, n : usize) -> ConnPeerResult {
        while (self.read_queue.len() < n) {
            self.read_more_bytes().await?;
        }
        Ok(())
    }

    async fn read_more_bytes(&mut self) -> ConnPeerResult {
        let mut buf   = [0u8; 64];
        match (self.stream.read(&mut buf).await?) {
            0     => Err(ConnPeerError::PeerClosed),
            count => {
                self.read_queue.extend(buf[0..count].iter()); // TODO: Decrypt
                Ok(())
            }
        }
    }

}


struct PacketDecompress {
    buf  : Box<[MaybeUninit<u8>]>,
    head : usize
}

impl Write for PacketDecompress {

    fn write(&mut self, buf : &[u8]) -> io::Result<usize> {
        if (self.head + buf.len() > self.buf.len()) {
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            // SAFETY: Checked destination validity in if-condition above.
            unsafe { ptr::copy_nonoverlapping(buf.as_ptr(), self.buf.as_mut_ptr().byte_add(self.head) as _, buf.len()); }
            Ok(buf.len())
        }
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> { Ok(()) }

}


pub struct ReadPacketContainer<P>
where
    P : PacketPrefixedDecode
{
    raw     : ManuallyDrop<Pin<Box<[u8]>>>,
    packet  : ManuallyDrop<P::Output<'static>>
}

impl<P> Deref for ReadPacketContainer<P>
where
    P : PacketPrefixedDecode
{
    type Target = P::Output<'static>;
    #[inline]
    fn deref(&self) -> &Self::Target { &self.packet }
}

// Forces `self.raw` to live until after `self.packet` is dropped.
impl<P> Drop for ReadPacketContainer<P>
where
    P : PacketPrefixedDecode
{
    fn drop(&mut self) {
        // SAFETY: `self.packet` still hasn't been dropped.
        unsafe { ManuallyDrop::drop(&mut self.packet); }
        // SAFETY: `self.raw` still hasn't been dropped.
        unsafe { ManuallyDrop::drop(&mut self.raw); }
    }
}

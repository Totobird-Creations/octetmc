use super::{ ConnPeerResult, ConnPeerError };
use crate::util::future::timeout;
use octetmc_protocol::value::varint::{ VarInt, VarIntDecodeError };
use octetmc_protocol::packet::{ BoundC2S, PacketBoundState, BufHead };
use octetmc_protocol::packet::decode::{ PacketDecode, PacketPartDecode, DecodeBuf, UnknownPrefix, MAX_PACKET_LENGTH };
use core::net::SocketAddr;
use core::time::Duration;
use core::mem::{ self, MaybeUninit, ManuallyDrop };
use core::ptr;
use core::pin::Pin;
use core::ops::Deref;
use std::collections::VecDeque;
use std::io::{ self, Write };
use std::borrow::Cow;
use smol::net::TcpStream;
use smol::io::AsyncReadExt;
use flate2::write::ZlibDecoder;



pub(super) struct ConnPeerComms {
    stream             : TcpStream,
    addr               : SocketAddr,
    read_queue         : VecDeque<u8>,
    compress_threshold : Option<usize>
}

impl ConnPeerComms {
    pub(super) fn new(stream : TcpStream, addr : SocketAddr) -> Self {
        Self { stream, addr,
            read_queue         : VecDeque::new(),
            compress_threshold : None
        }
    }
}

impl ConnPeerComms {

    pub(super) async fn read_packet<P>(&mut self) -> ConnPeerResult<ReadPacketContainer<P>>
    where
        P                                      : PacketDecode<Bound = BoundC2S>,
        (BoundC2S, <P as PacketDecode>::State) : PacketBoundState,
        for<'l> <P as PacketDecode>::Error<'l> : Into<Cow<'static, str>>
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
            if let Err(_) = z.write_all(unsafe { a.get_unchecked(0..consuming) }) {
                return Err(ConnPeerError::BadPacket(Cow::Borrowed("failed to decompress")));
            }
            let consuming = compressed_packet_len - consuming;
            // SAFETY: `consuming` can never be `b.len()` or higher, as `self.read_queue` has enough bytes for the whole packet (see <NOTE 1>).
            if let Err(_) = z.write_all(unsafe { b.get_unchecked(0..consuming) }) {
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
            // SAFETY:
            unsafe { ptr::copy_nonoverlapping(a.as_ptr(), buf.as_mut_ptr() as _, a.len()); }
            let consuming = decompressed_packet_len - consuming;
            // SAFETY: `consuming` can never be `b.len()` or higher, as `self.read_queue` has enough bytes for the whole packet (see <NOTE 1>).
            let b = unsafe { b.get_unchecked(0..consuming) };
            unsafe { ptr::copy_nonoverlapping(b.as_ptr(), buf[a.len()..].as_mut_ptr() as _, b.len()); }

            // SAFETY: All bytes in `buf` were written.
            unsafe { buf.assume_init() }
        });

        let packet = P::decode_prefixed(DecodeBuf::from(unsafe { mem::transmute::<&[u8], &[u8]>(&*buf) }), &mut BufHead::new()).map_err(|e| match (e) {
            UnknownPrefix::UnknownPrefix(p) => ConnPeerError::UnknownPacketPrefix(p),
            UnknownPrefix::Error(e)         => ConnPeerError::BadPacket(e.into()),
        })?;
        Ok(ReadPacketContainer {
            raw    : ManuallyDrop::new(buf),
            packet : ManuallyDrop::new(packet)
        })
    }

    pub(super) async fn read_packet_timeout<P>(&mut self, dur : Duration) -> ConnPeerResult<ReadPacketContainer<P>>
    where
        P                                      : PacketDecode<Bound = BoundC2S>,
        (BoundC2S, <P as PacketDecode>::State) : PacketBoundState,
        for<'l> <P as PacketDecode>::Error<'l> : Into<Cow<'static, str>>
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
            if (index >= VarInt::<u32>::MAX_BYTES) {
                return Err(ConnPeerError::InvalidPacketLength);
            }
            match (self.read_queue.pop_front()) {
                None => {
                    let mut buf1  = [0u8; 64];
                    let     count = self.stream.read(&mut buf1).await?;
                    self.read_queue.extend(buf1[0..count].into_iter());
                },
                Some(b) => {
                    buf[index] = b;
                    index += 1;
                    match (VarInt::<u32>::decode(DecodeBuf::from(&buf[0..index]), &mut BufHead::new())) {
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
                }
            }
        }
    }

    async fn wait_for_bytes(&mut self, n : usize) -> ConnPeerResult {
        while (self.read_queue.len() < n) {
            let mut buf   = [0u8; 64];
            let     count = self.stream.read(&mut buf).await?;
            self.read_queue.extend(buf[0..count].into_iter());
        }
        Ok(())
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
            unsafe { ptr::copy_nonoverlapping(buf.as_ptr(), self.buf.get_unchecked_mut(self.head..).as_mut_ptr() as _, buf.len()); }
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }

}


pub struct ReadPacketContainer<P>
where
    P                                      : PacketDecode<Bound = BoundC2S>,
    (BoundC2S, <P as PacketDecode>::State) : PacketBoundState
{
    raw     : ManuallyDrop<Pin<Box<[u8]>>>,
    packet  : ManuallyDrop<P::Output<'static>>
}

impl<P> Deref for ReadPacketContainer<P>
where
    P                                      : PacketDecode<Bound = BoundC2S>,
    (BoundC2S, <P as PacketDecode>::State) : PacketBoundState
{
    type Target = P::Output<'static>;
    fn deref(&self) -> &Self::Target { &self.packet }
}

// Forces `self.raw` to live until after `self.packet` is dropped.
impl<P> Drop for ReadPacketContainer<P>
where
    P                                      : PacketDecode<Bound = BoundC2S>,
    (BoundC2S, <P as PacketDecode>::State) : PacketBoundState
{
    fn drop(&mut self) {
        // SAFETY: `self.packet` still hasn't been dropped.
        unsafe { ManuallyDrop::drop(&mut self.packet); }
        // SAFETY: `self.raw` still hasn't been dropped.
        unsafe { ManuallyDrop::drop(&mut self.raw); }
    }
}

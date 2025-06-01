use super::{ ConnPeerComms, ConnPeerCrypters, MAX_READ_QUEUE_SIZE };
use crate::conn::{ ConnPeerResult, ConnPeerError };
use crate::util::future::timeout;
use octetmc_protocol::value::varint::{ VarInt, VarIntDecodeError };
use octetmc_protocol::packet::decode::{ PacketPrefixedDecode, PacketPartDecode, DecodeBufHead, DecodeBuf, UnknownPrefix };
use core::time::Duration;
use core::mem::{ self, MaybeUninit, ManuallyDrop };
use core::{ iter, ptr };
use core::pin::Pin;
use std::io::{ self, Write };
use std::borrow::Cow;
use smol::io::AsyncReadExt;
use flate2::write::ZlibDecoder;


const MAX_UNCOMPRESSED_PACKET_LEN : usize = 4096;


impl ConnPeerComms {

    #[inline]
    pub(crate) fn try_read_packet<P>(&mut self) -> ConnPeerResult<Option<P::Output<'_>>>
    where
        P : PacketPrefixedDecode
    { self.try_read_packet_inner::<P, _, _>(|bytes| Self::decode_packet::<P>(bytes)) }

    #[expect(unused)]
    #[inline]
    pub(crate) fn try_read_packet_boxed<P>(&mut self) -> ConnPeerResult<Option<ReadPacketBoxed<P>>>
    where
        P : PacketPrefixedDecode
    { self.try_read_packet_inner::<P, _, _>(|bytes| Self::decode_packet_boxed::<P>(bytes)) }

    fn try_read_packet_inner<'l, P, F, T>(&'l mut self, f : F) -> ConnPeerResult<Option<T>>
    where
        P : PacketPrefixedDecode,
        F : FnOnce(&'l [u8]) -> ConnPeerResult<T>
    {
        let Some((total_len, _,)) = self.try_read_packet_len()?
            else { return Ok(None); };
        if (self.read_queue.len() < total_len) { // <NOTE 1>
            return Ok(None);
        }
        f(unsafe { self.read_packet_content_data(total_len)? }).map(Some)
    }


    #[inline]
    pub(crate) async fn read_packet<P>(&mut self) -> ConnPeerResult<P::Output<'_>>
    where
        P : PacketPrefixedDecode
    { self.read_packet_inner::<P, _, _>(|bytes| Self::decode_packet::<P>(bytes)).await }

    #[inline]
    pub(crate) async fn read_packet_boxed<P>(&mut self) -> ConnPeerResult<ReadPacketBoxed<P>>
    where
        P : PacketPrefixedDecode
    { self.read_packet_inner::<P, _, _>(|bytes| Self::decode_packet_boxed::<P>(bytes)).await }

    async fn read_packet_inner<'l, P, F, T>(&'l mut self, f : F) -> ConnPeerResult<T>
    where
        P : PacketPrefixedDecode,
        F : FnOnce(&'l [u8]) -> ConnPeerResult<T>
    {
        let (total_len, _,) = self.read_packet_len().await?;
        self.wait_for_bytes(total_len).await?; // <NOTE 2>
        f(unsafe { self.read_packet_content_data(total_len)? })
    }


    pub(crate) async fn read_packet_timeout<P>(&mut self, dur : Duration) -> ConnPeerResult<P::Output<'_>>
    where
        P : PacketPrefixedDecode
    { Self::read_packet_timeout_inner::<P, _, _>(dur, self.read_packet::<P>()).await }

    #[inline]
    pub(crate) async fn read_packet_boxed_timeout<P>(&mut self, dur : Duration) -> ConnPeerResult<ReadPacketBoxed<P>>
    where
        P : PacketPrefixedDecode
    { Self::read_packet_timeout_inner::<P, _, _>(dur, self.read_packet_boxed::<P>()).await }

    async fn read_packet_timeout_inner<P, F, T>(dur : Duration, f : F) -> ConnPeerResult<T>
    where
        P : PacketPrefixedDecode,
        F : Future<Output = ConnPeerResult<T>>
    { match (timeout(dur, f).await) {
        Ok(Ok(out))  => Ok(out),
        Ok(Err(err)) => Err(err),
        Err(_)       => Err(ConnPeerError::TimedOut)
    } }


    fn decode_packet<P>(bytes : &[u8]) -> ConnPeerResult<P::Output<'_>>
    where
        P : PacketPrefixedDecode
    {
        let mut head = DecodeBufHead::default();
        let packet = P::decode_prefixed(DecodeBuf::from(bytes), &mut head).map_err(|e| match (e) {
            UnknownPrefix::UnknownPrefix(p) => ConnPeerError::UnknownPacketPrefix(p),
            UnknownPrefix::Error(e)         => ConnPeerError::BadPacket(e.into()),
        })?;
        if (head.consumed() < bytes.len()) { return Err(ConnPeerError::NoPacketEnd); }
        Ok(packet)
    }


    fn decode_packet_boxed<P>(bytes : &[u8]) -> ConnPeerResult<ReadPacketBoxed<P>>
    where
        P : PacketPrefixedDecode
    {
        let mut bytes_boxed = Box::<[u8]>::new_uninit_slice(bytes.len());
        // SAFETY: `bytes` and `bytes_boxed` have the same length.
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), bytes_boxed.as_mut_ptr() as _, bytes.len()); }
        // SAFETY: All elements were initialised in the line above.
        let bytes_boxed = Pin::new(unsafe { bytes_boxed.assume_init() });

        let mut head        = DecodeBufHead::default();
        // SAFETY: `bytes_boxed` is kept alive by `ReadPacketBoxed<P>`.
        let packet = P::decode_prefixed(DecodeBuf::from(unsafe { mem::transmute::<&[u8], &[u8]>(&*bytes_boxed) }), &mut head).map_err(|e| match (e) {
            UnknownPrefix::UnknownPrefix(p) => ConnPeerError::UnknownPacketPrefix(p),
            UnknownPrefix::Error(e)         => ConnPeerError::BadPacket(e.into()),
        })?;
        if (head.consumed() < bytes.len()) { return Err(ConnPeerError::NoPacketEnd); }
        Ok(ReadPacketBoxed {
            raw    : ManuallyDrop::new(bytes_boxed),
            packet : ManuallyDrop::new(packet)
        })
    }


    unsafe fn read_packet_content_data(&mut self, total_len : usize) -> ConnPeerResult<&[u8]> {
        match (self.compress_threshold) {

            None => {
                self.buf0.extend(iter::repeat_n(0u8,
                    total_len.saturating_sub(self.buf0.len())
                ));

                let (a, b,) = self.read_queue.as_slices();
                let consuming = total_len.min(a.len());
                // SAFETY: `consuming` is clamped down to `a.len()` on the line above.
                let a = unsafe { a.get_unchecked(0..consuming) };
                unsafe { ptr::copy_nonoverlapping(a.as_ptr(), self.buf0.as_mut_ptr(), a.len()); }
                let consuming = total_len - consuming;
                // SAFETY: `consuming` can never be `b.len()` or greater, as `self.read_queue` has enough bytes for the whole packet (see <NOTE 1> and <NOTE 2>).
                let b = unsafe { b.get_unchecked(0..consuming) };
                unsafe { ptr::copy_nonoverlapping(b.as_ptr(), self.buf0.as_mut_ptr().byte_add(a.len()), b.len()); }

                self.read_queue.drain(0..total_len);
                // SAFETY: `self.buf.len()` is greater than or equal to total_len.
                Ok(unsafe { self.buf0.get_unchecked(0..total_len) })
            },

            Some(threshold) => {
                // SAFETY: `self.read_queue` has enough bytes to read a `VarInt<u32>`.
                let (uncompressed_len, consumed,) = unsafe { self.try_read_packet_len()?.unwrap_unchecked() };
                println!("{}", uncompressed_len);
                match (uncompressed_len) {

                    0 => {
                        todo!()
                    },

                    1..=MAX_UNCOMPRESSED_PACKET_LEN => {
                        todo!()
                    },

                    _ => Err(ConnPeerError::PacketTooLong)
                }
            }

        }
    }


    /// Returns (length value, consumed byte count,).
    async fn read_packet_len(&mut self) -> ConnPeerResult<(usize, usize,)> {
        let mut buf   = [0u8; VarInt::<u32>::MAX_BYTES];
        let mut index = 0;
        loop {
            match (self.read_queue.get(index)) {
                None => { self.accept_more_bytes().await?; },
                Some(&b) => {
                    // SAFETY: index can never be greater than `VarInt::<u32>::MAX_BYTES`.
                    match (unsafe { self.try_decode_varintu32(&mut buf, &mut index, b) }) {
                        Ok(Some(out)) => { return Ok(out); },
                        Ok(None)      => { },
                        Err(err)      => { return Err(err); }
                    }
                }
            }
        }
    }

    /// Returns (length value, consumed byte count,).
    fn try_read_packet_len(&mut self) -> ConnPeerResult<Option<(usize, usize,)>> {
        let mut buf   = [0u8; VarInt::<u32>::MAX_BYTES];
        let mut index = 0;
        loop { match (self.read_queue.get(index)) {
            None => { return Ok(None); },
            Some(&b) => {
                // SAFETY: index can never be greater than `VarInt::<u32>::MAX_BYTES`.
                match (unsafe { self.try_decode_varintu32(&mut buf, &mut index, b) }) {
                    Ok(Some(out)) => { return Ok(Some(out)); },
                    Ok(None)      => { },
                    Err(err)      => { return Err(err); }
                }
            }
        } }
    }

    async fn wait_for_bytes(&mut self, n : usize) -> ConnPeerResult {
        while (self.read_queue.len() < n) {
            self.accept_more_bytes().await?;
        }
        Ok(())
    }

    pub(crate) async fn accept_more_bytes(&mut self) -> ConnPeerResult {
        let mut buf = [0u8; 64];
        match (self.stream.read(&mut buf).await?) {
            0     => Err(ConnPeerError::PeerClosed),
            count => {
                if ((self.read_queue.len() + count) > MAX_READ_QUEUE_SIZE) {
                    return Err(ConnPeerError::ReadQueueOverflow);
                }
                // Gives space for up to `block_size` 64.
                let mut decrypted_buf = [0u8; 128];
                let decrypted_buf = if let Some(ConnPeerCrypters { decrypter, .. }) = &mut self.crypters {
                    // SAFETY: `block_size` can not be greater than 64 (see `ConnPeerComs::set_crypters`)
                    let count = unsafe { decrypter.update_unchecked(
                        // SAFETY: count can never be greater than `buf.len()`.
                        buf.get_unchecked(0..count),
                        decrypted_buf.get_unchecked_mut(0..count)
                    ) }.unwrap();
                    // SAFETY: `count` can not be greater than `buf.len() + block_size`.
                    unsafe { decrypted_buf.get_unchecked(0..count) }
                } else { &buf };
                // SAFETY: count can never be greater than `buf.len()`.
                self.read_queue.extend(unsafe { decrypted_buf.get_unchecked(0..count) }.iter());
                Ok(())
            }
        }
    }


    unsafe fn try_decode_varintu32(&mut self,
        buf   : &mut [u8; VarInt::<u32>::MAX_BYTES],
        index : &mut usize,
        b     : u8
    ) -> ConnPeerResult<Option<(usize, usize,)>> {
        buf[*index] = b;
        *index += 1;
        match (VarInt::<u32>::decode(DecodeBuf::from(unsafe { buf.get_unchecked(0..(*index)) }), &mut DecodeBufHead::default())) {
            Err(VarIntDecodeError::IncompleteData) => {
                if (*index >= VarInt::<u32>::MAX_BYTES) {
                    Err(ConnPeerError::InvalidPacketLength)
                } else { Ok(None) }
            },
            Err(VarIntDecodeError::TooLong) => {
                Err(ConnPeerError::InvalidPacketLength)
            },
            Ok(value) => {
                let _ = self.read_queue.drain(0..(*index));
                let value = *value as usize;
                if (value > MAX_UNCOMPRESSED_PACKET_LEN) {
                    Err(ConnPeerError::PacketTooLong)
                } else { Ok(Some((value, *index,))) }
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


#[must_use]
pub struct ReadPacketBoxed<P>
where
    P : PacketPrefixedDecode
{
    raw    : ManuallyDrop<Pin<Box<[u8]>>>,
    packet : ManuallyDrop<P::Output<'static>>
}

impl<P> ReadPacketBoxed<P>
where
    P : PacketPrefixedDecode
{
    #[inline(always)]
    pub fn get<'l>(&'l self) -> &'l P::Output<'l> {
        // SAFETY: This just changes the lifetime so that it can not be used after `self` is dropped.
        unsafe { mem::transmute::<&P::Output<'static>, &P::Output<'l>>(&self.packet) }
    }
}

// Forces `self.raw` to live until after `self.packet` is dropped.
impl<P> Drop for ReadPacketBoxed< P>
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

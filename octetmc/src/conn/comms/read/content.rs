use super::{ ConnPeerComms, ConnPeerResult, ConnPeerError, MAX_UNCOMPRESSED_PACKET_LEN };
use core::{ iter, ptr };
use std::io::{self, Write};
use std::collections::VecDeque;
use flate2::write::ZlibDecoder;


impl ConnPeerComms {

    pub(super) unsafe fn read_packet_compressed_content(&mut self, total_len : usize) -> ConnPeerResult<&[u8]> {
        match (self.compress_threshold) {

            // Compression is not enabled.
            None => Ok(unsafe { Self::read_packet_content(&mut self.read_queue, &mut self.buf0, total_len) }),

            Some(_) => {
                // SAFETY: `self.read_queue` has enough bytes to read a `VarInt<u32>`.
                let (uncompressed_len, consumed,) = unsafe { self.try_read_packet_len()?.unwrap_unchecked() };
                let total_len = total_len - consumed;
                match (uncompressed_len) {

                    // Compression is enabled but this packet is not compressed.
                    0 => Ok(unsafe { Self::read_packet_content(&mut self.read_queue, &mut self.buf0, total_len) }),

                    // Compression is enabled and this packet is compressed.
                    1..=MAX_UNCOMPRESSED_PACKET_LEN => {
                        self.buf1.clear();
                        self.buf1.reserve_exact(uncompressed_len);
                        let mut de = PacketDecompress::new(&mut self.buf1, uncompressed_len);
                        let mut z  = ZlibDecoder::new(&mut de);
                        if (
                            z.write_all(unsafe { Self::read_packet_content(&mut self.read_queue, &mut self.buf0, total_len) }).is_err()
                            || z.try_finish().is_err()
                        ) { return Err(if (z.get_ref().too_long) { ConnPeerError::PacketTooLong } else { ConnPeerError::BadPacketZlib }); }
                        drop(z);
                        Ok(&self.buf1)
                    },

                    // Decompressed packet is too long.
                    _ => Err(ConnPeerError::PacketTooLong)
                }
            }

        }
    }


    unsafe fn read_packet_content<'l>(read_queue : &mut VecDeque<u8>, buf : &'l mut Vec<u8>, total_len : usize) -> &'l [u8] {
        buf.extend(iter::repeat_n(0u8,
            total_len.saturating_sub(buf.len())
        ));

        let (a, b,) = read_queue.as_slices();
        let consuming = total_len.min(a.len());
        // SAFETY: `consuming` is clamped down to `a.len()` on the line above.
        let a = unsafe { a.get_unchecked(0..consuming) };
        unsafe { ptr::copy_nonoverlapping(a.as_ptr(), buf.as_mut_ptr(), a.len()); }
        let consuming = total_len - consuming;
        // SAFETY: `consuming` can never be `b.len()` or greater, as `self.read_queue` has enough bytes for the whole packet (see <NOTE 1> and <NOTE 2>).
        let b = unsafe { b.get_unchecked(0..consuming) };
        unsafe { ptr::copy_nonoverlapping(b.as_ptr(), buf.as_mut_ptr().byte_add(a.len()), b.len()); }

        read_queue.drain(0..total_len);
        // SAFETY: `self.buf.len()` is greater than or equal to total_len.
        unsafe { buf.get_unchecked(0..total_len) }
    }

}


struct PacketDecompress<'l> {
    buf      : &'l mut Vec<u8>,
    max_len  : usize,
    too_long : bool
}

impl<'l> PacketDecompress<'l> {
    fn new(buf : &'l mut Vec<u8>, max_len : usize) -> Self { Self {
        buf, max_len, too_long : false
    } }
}

impl io::Write for PacketDecompress<'_> {

    fn write(&mut self, buf : &[u8]) -> io::Result<usize> {
        if (self.buf.len() + buf.len() > self.max_len) {
            self.too_long = true;
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            self.buf.extend_from_slice(buf);
            Ok(buf.len())
        }
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> { Ok(()) }

}

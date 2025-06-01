use super::{ ConnPeerComms, ConnPeerCrypters };
use crate::conn::ConnPeerResult;
use octetmc_protocol::value::varint::VarInt;
use octetmc_protocol::packet::encode::{ PacketPrefixedEncode, EncodeBuf };
use core::{ iter, ptr };
use std::io::Write;
use smol::io::AsyncWriteExt;
use flate2::Compression;
use flate2::write::ZlibEncoder;


const COMPRESSION : Compression = Compression::new(6);


impl ConnPeerComms {

    pub(crate) async fn send_packet<P>(&mut self, packet : &P) -> ConnPeerResult
    where
        P : PacketPrefixedEncode
    {
        const VARINT32_SIZE   : usize = VarInt::<u32>::MAX_BYTES;
        const VARINT32_SIZE_2 : usize = VARINT32_SIZE * 2;

        // Make sure `self.write_buf0` has enough space.
        self.buf0.clear();
        self.buf0.reserve_exact(VARINT32_SIZE_2 + P::predict_size(packet));
        self.buf0.extend_from_slice(&[0u8; VARINT32_SIZE_2]);

        // Encode the packet.
        P::encode_prefixed(packet, &mut EncodeBuf::from(&mut self.buf0));
        let unaltered_packet_len = self.buf0.len() - VARINT32_SIZE_2;

        // Compress the packet (if necessary) and attach length to the front.
        let compressed_packet = match (self.compress_threshold) {

            None => {
                // SAFETY: `self.write_buf0` contains at least `VARINT32_SIZE_2` items.
                let dst_end = unsafe { self.buf0.as_mut_ptr().byte_add(VARINT32_SIZE_2) };
                let len     = unsafe { Self::write_varint32_before(unaltered_packet_len, dst_end) };
                unsafe { self.buf0.get_unchecked((VARINT32_SIZE_2 - len)..) }
            },

            Some(threshold) => {

                // Packet is short enough. Skip compression.
                if (unaltered_packet_len < threshold) {
                    // SAFETY: `self.write_buf0` contains at least `VARINT32_SIZE_2` items.
                    let dst0_end = unsafe { self.buf0.as_mut_ptr().byte_add(VARINT32_SIZE_2) };
                    let len0     = unsafe { Self::write_varint32_before(0, dst0_end) };
                    let dst1_end = unsafe { dst0_end.byte_sub(len0) };
                    let len1     = unsafe { Self::write_varint32_before(unaltered_packet_len + len0, dst1_end) };
                    unsafe { self.buf0.get_unchecked((VARINT32_SIZE_2 - len0 - len1)..) }
                }

                else {
                    // SAFETY: `self.write_buf0` contains at least `VARINT32_SIZE_2` items.
                    let uncompressed_packet = unsafe { self.buf0.get_unchecked(VARINT32_SIZE_2..) };
                    self.buf1.clear();
                    self.buf1.reserve_exact(VARINT32_SIZE_2 + uncompressed_packet.len());
                    self.buf1.extend_from_slice(&[0u8; VARINT32_SIZE_2]);
                    let mut z = ZlibEncoder::new(&mut self.buf1, COMPRESSION);
                    _ = z.write_all(uncompressed_packet); // These errors can be ignored because writing to a `Vec` can not fail.
                    _ = z.finish();
                    let compressed_packet_len = self.buf1.len() - VARINT32_SIZE_2;
                    let dst0_end = unsafe { self.buf1.as_mut_ptr().byte_add(VARINT32_SIZE_2) };
                    let len0     = unsafe { Self::write_varint32_before(unaltered_packet_len, dst0_end) };
                    let dst1_end = unsafe { dst0_end.byte_sub(len0) };
                    let len1     = unsafe { Self::write_varint32_before(compressed_packet_len + len0, dst1_end) };
                    unsafe { self.buf1.get_unchecked((VARINT32_SIZE_2 - len0 - len1)..) }
                }

            }

        };

        // Encrypt the packet (if necessary) and send it to the client.
        match (&mut self.crypters) {

            None => { self.stream.write_all(compressed_packet).await?; },

            Some(ConnPeerCrypters { encrypter, block_size, .. }) => {
                self.buf2.extend(iter::repeat_n(0u8,
                    (compressed_packet.len() + *block_size).saturating_sub(self.buf2.len())
                ));
                // SAFETY: `self.write_buf2.len()` is greater than `compressed_packet.len() + block_size`, guaranteed by the previous line.
                let count = unsafe { encrypter.update_unchecked(compressed_packet, &mut self.buf2) }.unwrap();
                // SAFETY: `self.write_buf2` contains at least `count` items.
                self.stream.write_all(unsafe { self.buf2.get_unchecked(0..count) }).await?;
            }

        }
        self.stream.flush().await?;

        Ok(())
    }

    unsafe fn write_varint32_before(value : usize, dst_end : *mut u8) -> usize {
        let mut buf = [0u8; VarInt::<u32>::MAX_BYTES];
        let     buf = VarInt::<u32>::from(value as u32).encode_as_slice(&mut buf);
        let     dst = unsafe { dst_end.byte_sub(buf.len()) };
        unsafe { ptr::copy_nonoverlapping(buf.as_ptr(), dst, buf.len()); }
        buf.len()
    }

}

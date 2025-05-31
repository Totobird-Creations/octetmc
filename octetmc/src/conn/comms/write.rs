use super::ConnPeerComms;
use crate::conn::ConnPeerResult;
use octetmc_protocol::value::varint::VarInt;
use octetmc_protocol::packet::encode::{ PacketPrefixedEncode, EncodeBuf };
use smol::io::AsyncWriteExt;


impl ConnPeerComms {

    pub(crate) async fn send_packet<P>(&mut self, packet : &P) -> ConnPeerResult
    where
        P : PacketPrefixedEncode
    {
        let mut packet_data_buf = EncodeBuf::default();
        packet_data_buf.reserve(P::predict_size(packet));
        P::encode_prefixed(packet, &mut packet_data_buf);

        let uncompressed_packet_len = packet_data_buf.len();
        let is_compressed           = self.compress_threshold.is_some_and(|ct| uncompressed_packet_len >= ct);

        if (is_compressed) {

            // let mut packet_len_buf    = Default::default();
            // let     varint_packet_len = VarInt::<u32>::from(uncompressed_packet_len as u32).encode_as_slice(&mut packet_len_buf);
            // let mut final_buf         = Box::<[u8]>::new_uninit_slice(buf.len () + varint_packet_len.len());

            // // SAFETY: `final_buf` has exactly enough space for `packet_len_buf` and `buf`.
            // unsafe { ptr::copy_nonoverlapping(packet_len_buf.as_ptr(), final_buf.as_mut_ptr() as _, packet_len_buf.len()); }
            // unsafe { ptr::copy_nonoverlapping(buf.as_bytes().as_ptr(), final_buf.as_mut_ptr().byte_add(packet_len_buf.len()) as _, buf.len()); }

            todo!()

        } else {

            let mut packet_len_buf    = Default::default();
            let     varint_packet_len = VarInt::<u32>::from(uncompressed_packet_len as u32).encode_as_slice(&mut packet_len_buf);

            self.stream.write_all(varint_packet_len).await?;
            self.stream.write_all(packet_data_buf.as_bytes()).await?;
            self.stream.flush().await?;

            Ok(())
        }

    }

}

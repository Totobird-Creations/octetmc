use super::{ PacketPartEncode, EncodeBuf };
use crate::value::varint::VarInt;


impl PacketPartEncode for str {

    fn encode(&self, buf : &mut EncodeBuf) {
        let mut len_buf = Default::default();
        VarInt::<u32>::from(self.len() as u32).encode_as_slice(&mut len_buf);
        buf.reserve(len_buf.len() + self.len());
        buf.write_n(&len_buf);
        buf.write_n(self.as_bytes());
    }

}

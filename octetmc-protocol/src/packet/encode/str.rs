use super::{ PacketPartEncode, EncodeBuf };
use crate::value::varint::VarInt;


impl PacketPartEncode for str {

    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(&VarInt::<u32>::from(self.len() as u32));
        buf.write_n(self.as_bytes());
    }

}

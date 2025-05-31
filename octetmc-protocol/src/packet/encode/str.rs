use super::{ PacketPartEncode, EncodeBuf };
use crate::value::varint::VarInt;


impl PacketPartEncode for &str {

    #[inline]
    fn predict_size(&self) -> usize {
        VarInt::<u32>::from(self.len() as u32).predict_size() + self.len()
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        let mut len_buf = Default::default();
        let     len_buf = VarInt::<u32>::from(self.len() as u32).encode_as_slice(&mut len_buf);
        buf.write_n(&len_buf);
        buf.write_n(self.as_bytes());
    }

}


impl PacketPartEncode for str {

    #[inline(always)]
    fn predict_size(&self) -> usize {
        <&str as PacketPartEncode>::predict_size(&self)
    }

    #[inline(always)]
    fn encode(&self, buf : &mut EncodeBuf) {
        <&str as PacketPartEncode>::encode(&self, buf)
    }

}


impl PacketPartEncode for String {

    #[inline(always)]
    fn predict_size(&self) -> usize {
        <str as PacketPartEncode>::predict_size(self.as_str())
    }

    #[inline(always)]
    fn encode(&self, buf : &mut EncodeBuf) {
        <str as PacketPartEncode>::encode(self.as_str(), buf)
    }

}

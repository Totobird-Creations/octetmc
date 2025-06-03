//! `0x03` `login_compression`


use crate::value::varint::VarInt;
use crate::packet::StateLogin;
use crate::packet::encode::{ EncodeBuf, PacketEncode };


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Encryption_Request
#[derive(Debug, Clone, Copy)]
pub struct LoginCompressionS2CLoginPacket {

    /// Minimum size of a packet for it to be compressed.
    ///
    /// Packets larger than or equal to the threshold are compressed.
    ///
    /// Packets are compressed using the ZLib algorithm.
    pub threshold : u32

}


impl PacketEncode for LoginCompressionS2CLoginPacket {
    type State = StateLogin;

    const PREFIX : u8 = 0x03;

    #[inline(always)]
    fn predict_size(&self) -> usize {
        VarInt::<u32>::MAX_BYTES
    }

    #[inline]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(VarInt::<u32>::from(self.threshold));
    }
}

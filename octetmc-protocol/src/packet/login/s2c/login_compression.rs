//! `0x03` `login_compression`


use crate::value::varint::VarInt;
use crate::packet::{ Packet, StateLogin };
use crate::packet::encode::{ EncodeBuf, PacketEncode };
use crate::packet::prefix_check::prefix_check_login_s2c;


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Encryption_Request>
#[derive(Debug, Clone, Copy)]
pub struct LoginCompressionS2CLoginPacket {

    /// Minimum size of a packet for it to be compressed.
    ///
    /// Packets larger than or equal to the threshold are compressed.
    ///
    /// Packets are compressed using the ZLib algorithm.
    pub threshold : u32

}


impl crate::Sealed for LoginCompressionS2CLoginPacket { }

impl Packet for LoginCompressionS2CLoginPacket { }


impl PacketEncode for LoginCompressionS2CLoginPacket {
    type State = StateLogin;

    const PREFIX : u8 = prefix_check_login_s2c!(login_compression, 0x03);

    #[inline(always)]
    fn predict_size(&self) -> usize {
        VarInt::<u32>::MAX_BYTES
    }

    #[inline]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(VarInt::<u32>::from(self.threshold));
    }
}

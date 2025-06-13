//! `0x01` `pong_response`


use crate::packet::{ Packet, StateStatus };
use crate::packet::encode::{ EncodeBuf, PacketEncode };


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Pong_Response_(status)>
#[derive(Debug, Clone, Copy)]
pub struct PongResponseS2CStatusPacket {

    /// The timestamp the client sent in `PingRequestS2CStatusPacket`.
    pub timestamp : u64

}


impl crate::Sealed for PongResponseS2CStatusPacket { }

impl Packet for PongResponseS2CStatusPacket { }


impl PacketEncode for PongResponseS2CStatusPacket {
    type State = StateStatus;

    const PREFIX : u8 = 0x01;

    #[inline(always)]
    fn predict_size(&self) -> usize {
        size_of::<u64>()
    }

    #[inline]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write(self.timestamp);
    }
}

//! `0x01` `ping_request`


use crate::packet::{ Packet, StateStatus };
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode, IncompleteData };


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Ping_Request_(status)>
#[derive(Debug, Clone, Copy)]
pub struct PingRequestC2SStatusPacket {

    /// May be any number, but vanilla clients will use the timestamp in milliseconds.
    pub timestamp : u64

}


impl crate::Sealed for PingRequestC2SStatusPacket { }

impl Packet for PingRequestC2SStatusPacket { }


impl PacketDecode for PingRequestC2SStatusPacket {
    type State = StateStatus;

    const PREFIX : u8 = 0x01;
    type Output<'l> = PingRequestC2SStatusPacket;
    type Error<'l>  = IncompleteData;

    #[inline]
    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    { Ok(Self { timestamp : buf.read_decode::<u64>(head)? }) }
}

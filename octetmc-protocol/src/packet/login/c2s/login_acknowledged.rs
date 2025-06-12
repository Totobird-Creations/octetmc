//! `0x03` `login_acknowledged`


use crate::packet::{ Packet, StateLogin };
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode, IncompleteData };


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Acknowledged>
#[derive(Debug, Clone, Copy)]
pub struct LoginAcknowledgedC2SLoginPacket;


impl crate::Sealed for LoginAcknowledgedC2SLoginPacket { }

impl Packet for LoginAcknowledgedC2SLoginPacket { }


impl PacketDecode for LoginAcknowledgedC2SLoginPacket {
    type State = StateLogin;

    const PREFIX : u8 = 0x03;
    type Output<'l> = LoginAcknowledgedC2SLoginPacket;
    type Error<'l>  = IncompleteData;

    #[inline(always)]
    fn decode<'l>(_buf : DecodeBuf<'l>, _head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    { Ok(Self) }
}

//! `0x03` `login_acknowledged`


use crate::packet::StateLogin;
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode, IncompleteData };


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login_Acknowledged
#[derive(Debug, Clone)]
pub struct LoginAcknowledgedC2SLoginPacket;


impl PacketDecode for LoginAcknowledgedC2SLoginPacket {
    type State = StateLogin;

    const PREFIX : u8 = 0x03;
    type Output<'l> = LoginAcknowledgedC2SLoginPacket;
    type Error<'l>  = IncompleteData;

    fn decode<'l>(_buf : DecodeBuf<'l>, _head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(Self)
    }
}

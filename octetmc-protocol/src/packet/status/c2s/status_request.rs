use crate::packet::StateStatus;
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode, IncompleteData };


#[derive(Debug, Clone)]
pub struct StatusRequestC2SStatusPacket;


impl PacketDecode for StatusRequestC2SStatusPacket {
    type State = StateStatus;

    const PREFIX : u8 = 0x00;
    type Output<'l> = StatusRequestC2SStatusPacket;
    type Error<'l>  = IncompleteData;

    fn decode<'l>(_buf : DecodeBuf<'l>, _head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(Self)
    }
}

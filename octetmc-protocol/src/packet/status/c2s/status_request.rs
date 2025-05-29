use crate::packet::{ BoundC2S, StateStatus, BufHead };
use crate::packet::decode::{ DecodeBuf, PacketDecode, IncompleteData };


#[derive(Debug, Clone)]
pub struct StatusRequestC2SStatusPacket;


impl PacketDecode for StatusRequestC2SStatusPacket {
    type Bound = BoundC2S;
    type State = StateStatus;

    const PREFIX : u8 = 0x00;
    type Output<'l> = StatusRequestC2SStatusPacket;
    type Error<'l>  = IncompleteData;

    fn decode<'l>(_buf : DecodeBuf<'l>, _head : &mut BufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(Self)
    }
}

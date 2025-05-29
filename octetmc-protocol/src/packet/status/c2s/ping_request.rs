use crate::packet::{ BoundC2S, StateStatus, BufHead };
use crate::packet::decode::{ DecodeBuf, PacketDecode, IncompleteData };


#[derive(Debug, Clone)]
pub struct PingRequestC2SStatusPacket {
    pub timestamp : u64
}


impl PacketDecode for PingRequestC2SStatusPacket {
    type Bound = BoundC2S;
    type State = StateStatus;

    const PREFIX : u8 = 0x00;
    type Output<'l> = PingRequestC2SStatusPacket;
    type Error<'l>  = IncompleteData;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut BufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(Self { timestamp : buf.read_decode::<u64>(head)? })
    }
}

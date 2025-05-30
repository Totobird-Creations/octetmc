use crate::packet::StateStatus;
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode, IncompleteData };


#[derive(Debug, Clone)]
pub struct PingRequestC2SStatusPacket {
    pub timestamp : u64
}


impl PacketDecode for PingRequestC2SStatusPacket {
    type State = StateStatus;

    const PREFIX : u8 = 0x01;
    type Output<'l> = PingRequestC2SStatusPacket;
    type Error<'l>  = IncompleteData;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(Self { timestamp : buf.read_decode::<u64>(head)? })
    }
}

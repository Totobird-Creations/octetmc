use crate::value::varint::VarInt;
use crate::packet::{ BoundC2S, StateHandshake };
use crate::packet::decode::{ DecodeBuf, DecodeError, PacketDecode };
use std::borrow::Cow;


pub struct IntentionC2SHandshakePacket<'l> {
    pub protocol  : u32,
    pub address   : Cow<'l, str>,
    pub port      : u16,
    pub intention : Intention
}


impl PacketDecode for IntentionC2SHandshakePacket<'static> {
    type Bound = BoundC2S;
    type State = StateHandshake;
    fn decode(buf : &mut DecodeBuf<'_>) -> Result<Self, DecodeError> {
        Ok(Self {
            protocol  : *buf.read_decode::<VarInt::<u32>>()?,
            address   : Cow::Owned(buf.read_decode::<String>()?),
            port      : buf.read_decode::<u16>()?,
            intention : Intention::try_from(buf.read_decode::<VarInt::<u32>>()?)?
        })
    }
}


pub enum Intention {
    Status   = 1,
    Login    = 2,
    Transfer = 3,
}
impl TryFrom<VarInt<u32>> for Intention {
    type Error = DecodeError;
    fn try_from(value : VarInt<u32>) -> Result<Self, Self::Error> {
        match (*value) {
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            3 => Ok(Self::Transfer),
            _ => Err(DecodeError::UnknownIntention)
        }
    }
}

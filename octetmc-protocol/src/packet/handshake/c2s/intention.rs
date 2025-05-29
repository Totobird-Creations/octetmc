use crate::value::varint::{ VarInt, VarIntDecodeError };
use crate::packet::{ BoundC2S, StateHandshake, BufHead };
use crate::packet::decode::{ DecodeBuf, PacketDecode, IncompleteData };
use crate::packet::decode::string::StringDecodeError;
use std::borrow::Cow;


#[derive(Debug, Clone)]
pub struct IntentionC2SHandshakePacket<'l> {
    pub protocol  : u32,
    pub address   : Cow<'l, str>,
    pub port      : u16,
    pub intention : Intention
}


impl PacketDecode for IntentionC2SHandshakePacket<'_> {
    type Bound = BoundC2S;
    type State = StateHandshake;

    const PREFIX : u8 = 0x00;
    type Output<'l> = IntentionC2SHandshakePacket<'l>;
    type Error<'l>  = IntentionDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut BufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(Self::Output {
            protocol  : *buf.read_decode::<VarInt::<u32>>(head)?,
            address   : Cow::Borrowed(buf.read_decode::<&str>(head)?),
            port      : buf.read_decode::<u16>(head)?,
            intention : Intention::try_from(buf.read_decode::<VarInt::<u32>>(head)?)?
        })
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Intention {
    Status   = 1,
    Login    = 2,
    Transfer = 3,
}
impl TryFrom<VarInt<u32>> for Intention {
    type Error = UnknownIntention;
    fn try_from(value : VarInt<u32>) -> Result<Self, Self::Error> {
        match (*value) {
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            3 => Ok(Self::Transfer),
            _ => Err(UnknownIntention)
        }
    }
}


pub enum IntentionDecodeError {
    IncompleteData,
    VarIntTooLong,
    StringInvalidUtf8,
    UnknownIntention
}

impl From<IntentionDecodeError> for Cow<'static, str> {
    fn from(value : IntentionDecodeError) -> Self { match (value) {
        IntentionDecodeError::IncompleteData    => IncompleteData.into(),
        IntentionDecodeError::VarIntTooLong     => VarIntDecodeError::TooLong.into(),
        IntentionDecodeError::StringInvalidUtf8 => StringDecodeError::InvalidUtf8.into(),
        IntentionDecodeError::UnknownIntention  => Self::Borrowed("unknown intention")
    } }
}

impl From<IncompleteData> for IntentionDecodeError {
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

impl From<VarIntDecodeError> for IntentionDecodeError {
    fn from(value : VarIntDecodeError) -> Self { match (value) {
        VarIntDecodeError::IncompleteData => Self::IncompleteData,
        VarIntDecodeError::TooLong        => Self::VarIntTooLong
    } }
}

impl From<StringDecodeError> for IntentionDecodeError {
    fn from(value : StringDecodeError) -> Self { match (value) {
        StringDecodeError::IncompleteData => Self::IncompleteData,
        StringDecodeError::VarIntTooLong  => Self::VarIntTooLong,
        StringDecodeError::InvalidUtf8    => Self::StringInvalidUtf8
    } }
}

impl From<UnknownIntention> for IntentionDecodeError {
    fn from(_ : UnknownIntention) -> Self { Self::UnknownIntention }
}


pub struct UnknownIntention;

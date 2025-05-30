//! `0x00` `intention`


use crate::value::varint::{ VarInt, VarIntDecodeError };
use crate::packet::{ StateHandshake };
use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketDecode, IncompleteData };
use crate::packet::decode::str::StringDecodeError;
use std::borrow::Cow;


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshake
#[derive(Debug, Clone)]
pub struct IntentionC2SHandshakePacket<'l> {

    /// The protocol version this server is running.
    ///
    /// https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol_version_numbers
    pub protocol  : u32,

    /// The hostname that was used to connect.
    pub address   : &'l str,

    /// The port that was used to connect.
    pub port      : u16,

    /// The client's intention.
    pub intention : Intention

}


impl PacketDecode for IntentionC2SHandshakePacket<'_> {
    type State = StateHandshake;

    const PREFIX : u8 = 0x00;
    type Output<'l> = IntentionC2SHandshakePacket<'l>;
    type Error<'l>  = IntentionDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(Self::Output {
            protocol  : *buf.read_decode::<VarInt::<u32>>(head)?,
            address   : buf.read_decode::<&str>(head)?,
            port      : buf.read_decode::<u16>(head)?,
            intention : Intention::try_from(buf.read_decode::<VarInt::<u32>>(head)?)?
        })
    }
}


/// A client's intention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Intention {

    /// The client will request server list information.
    Status   = 1,

    /// The client will try to join the game.
    Login    = 2,

    /// The client will try to join the game.
    ///
    /// Another server transferred the client to this server.
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


/// An `IntentionC2SHandshakePacket` failed to decode.
pub enum IntentionDecodeError {

    /// Not enough bytes were present.
    IncompleteData,

    /// The byte-encoded `VarInt` contained more than `MAX_BYTES` bytes.
    VarIntTooLong,

    /// A string contained invalid UTF-8.
    StringInvalidUtf8,

    /// The client requested an unknown intention.
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


/// The client requested an unknown intention.
pub struct UnknownIntention;

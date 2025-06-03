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
    pub address   : Cow<'l, str>,

    /// The port that was used to connect.
    pub port      : u16,

    /// The client's intention.
    pub intention : Intention

}


impl<'l> IntentionC2SHandshakePacket<'l> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `IntentionC2SHandshakePacket<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> IntentionC2SHandshakePacket<'static> {
        IntentionC2SHandshakePacket {
            protocol  : self.protocol,
            address   : Cow::Owned(self.address.into_owned()),
            port      : self.port,
            intention : self.intention
        }
    }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `IntentionC2SHandshakePacket<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> IntentionC2SHandshakePacket<'static> {
        IntentionC2SHandshakePacket {
            protocol  : self.protocol,
            address   : Cow::Owned((*self.address).to_owned()),
            port      : self.port,
            intention : self.intention
        }
    }

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
            address   : Cow::Borrowed(buf.read_decode::<&str>(head)?),
            port      : buf.read_decode::<u16>(head)?,
            intention : (match (*buf.read_decode::<VarInt::<u32>>(head)?) {
                1 => Intention::Status,
                2 => Intention::Login,
                3 => Intention::Transfer,
                v => { return Err(IntentionDecodeError::UnknownIntention(v)); }
            })
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


/// An `IntentionC2SHandshakePacket` failed to decode.
pub enum IntentionDecodeError {

    /// Not enough bytes were present.
    IncompleteData,

    /// The byte-encoded `VarInt` contained more than `MAX_BYTES` bytes.
    VarIntTooLong,

    /// A string contained invalid UTF-8.
    StringInvalidUtf8,

    /// The client requested an unknown intention.
    UnknownIntention(u32)

}

impl From<IntentionDecodeError> for Cow<'static, str> {
    fn from(value : IntentionDecodeError) -> Self { match (value) {
        IntentionDecodeError::IncompleteData      => IncompleteData.into(),
        IntentionDecodeError::VarIntTooLong       => VarIntDecodeError::TooLong.into(),
        IntentionDecodeError::StringInvalidUtf8   => StringDecodeError::InvalidUtf8.into(),
        IntentionDecodeError::UnknownIntention(_) => Self::Borrowed("unknown intention")
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

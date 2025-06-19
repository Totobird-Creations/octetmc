//! Packets sent by the client to the server in the config state.


use crate::value::varint::VarIntDecodeError;
use crate::value::ident::IdentDecodeError;
use crate::value::client_info::{ ClientInfoDecodeError };
use crate::packet::StateConfig;
use crate::packet::decode::{ IncompleteData, packet_decode_group };
use crate::packet::decode::str::StringDecodeError;
use std::borrow::Cow;


pub mod client_information;

// TODO: bookie_response

pub mod custom_payload;

pub mod finish_configuration;

// TODO: keep_alive

// TODO: pong

// TODO: resource_pack

// TODO: select_known_packs

// TODO: custom_click_action


packet_decode_group!{
    type State     = StateConfig;
    type Error<'l> = C2SConfigPacketDecodeError;
    /// `C2SConfig`-type packets.
    pub enum C2SConfigPackets<'l> {
        /// `ClientInformationC2SConfigPacket`
        ClientInformation(client_information::ClientInformationC2SConfigPacket<'l>),
        /// `CustomPayloadC2SConfigPacket`
        CustomPayload(custom_payload::CustomPayloadC2SConfigPacket<'l>),
        /// `FinishConfigurationC2SConfigPacket`
        FinishConfiguration(finish_configuration::FinishConfigurationC2SConfigPacket)
    }
}


impl C2SConfigPackets<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `C2SConfigPackets<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> C2SConfigPackets<'static> { match (self) {
        Self::ClientInformation   (v) => C2SConfigPackets::ClientInformation   (v.into_static_owned()),
        Self::CustomPayload       (v) => C2SConfigPackets::CustomPayload       (v.into_static_owned()),
        Self::FinishConfiguration (v) => C2SConfigPackets::FinishConfiguration (v),
    } }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `C2SConfigPackets<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> C2SConfigPackets<'static> { match (self) {
        Self::ClientInformation   (v) => C2SConfigPackets::ClientInformation   (v.to_static_owned()),
        Self::CustomPayload       (v) => C2SConfigPackets::CustomPayload       (v.to_static_owned()),
        Self::FinishConfiguration (v) => C2SConfigPackets::FinishConfiguration (*v),
    } }

}


/// A `C2SConfigPackets` failed to decode.
pub enum C2SConfigPacketDecodeError {

    /// Not enough bytes were present.
    IncompleteData,

    /// The byte-encoded `VarInt` contained more than `MAX_BYTES` bytes.
    VarIntTooLong,

    /// A string contained invalid UTF-8.
    InvalidUtf8,
    /// The identifier has an empty namespace.

    EmptyNspace,

    /// The identifier namespace contains an invalid character.
    BadNspaceChar(char),

    /// The identifier has an empty path part.s
    EmptyPathPart,

    /// The identifier path contains an invalid character.
    BadPathChar(char),

    /// The client declared a locale longer than 16 bytes.
    LocaleTooLong,

    /// View distance is zero.
    ZeroViewDist,

    /// The client declared an unknown chat mode.
    UnknownChatMode(u32),

    /// The client declared an unknown main hand.
    UnknownMainHand(u32),

    /// The client declared an unknown particle status.
    UnknownParticleStatus(u32)

}

impl From<C2SConfigPacketDecodeError> for Cow<'static, str> {
    fn from(value : C2SConfigPacketDecodeError) -> Self { match (value) {
        C2SConfigPacketDecodeError::IncompleteData           => IncompleteData.into(),
        C2SConfigPacketDecodeError::VarIntTooLong            => VarIntDecodeError::TooLong.into(),
        C2SConfigPacketDecodeError::InvalidUtf8              => StringDecodeError::InvalidUtf8.into(),
        C2SConfigPacketDecodeError::EmptyNspace              => IdentDecodeError::EmptyNspace.into(),
        C2SConfigPacketDecodeError::BadNspaceChar(ch)        => IdentDecodeError::BadNspaceChar(ch).into(),
        C2SConfigPacketDecodeError::EmptyPathPart            => IdentDecodeError::EmptyPathPart.into(),
        C2SConfigPacketDecodeError::BadPathChar(ch)          => IdentDecodeError::BadPathChar(ch).into(),
        C2SConfigPacketDecodeError::LocaleTooLong            => ClientInfoDecodeError::LocaleTooLong.into(),
        C2SConfigPacketDecodeError::ZeroViewDist             => ClientInfoDecodeError::ZeroViewDist.into(),
        C2SConfigPacketDecodeError::UnknownChatMode(v)       => C2SConfigPacketDecodeError::UnknownChatMode(v).into(),
        C2SConfigPacketDecodeError::UnknownMainHand(v)       => C2SConfigPacketDecodeError::UnknownMainHand(v).into(),
        C2SConfigPacketDecodeError::UnknownParticleStatus(v) => C2SConfigPacketDecodeError::UnknownParticleStatus(v).into()
    } }
}

impl From<IdentDecodeError> for C2SConfigPacketDecodeError {
    fn from(value : IdentDecodeError) -> Self { match (value) {
        IdentDecodeError::IncompleteData    => Self::IncompleteData,
        IdentDecodeError::VarIntTooLong     => Self::VarIntTooLong,
        IdentDecodeError::InvalidUtf8       => Self::InvalidUtf8,
        IdentDecodeError::EmptyNspace       => Self::EmptyNspace,
        IdentDecodeError::BadNspaceChar(ch) => Self::BadNspaceChar(ch),
        IdentDecodeError::EmptyPathPart     => Self::EmptyPathPart,
        IdentDecodeError::BadPathChar(ch)   => Self::BadPathChar(ch)
    } }
}

impl From<ClientInfoDecodeError> for C2SConfigPacketDecodeError {
    fn from(value : ClientInfoDecodeError) -> Self { match (value) {
        ClientInfoDecodeError::IncompleteData           => Self::IncompleteData,
        ClientInfoDecodeError::VarIntTooLong            => Self::VarIntTooLong,
        ClientInfoDecodeError::InvalidUtf8              => Self::InvalidUtf8,
        ClientInfoDecodeError::LocaleTooLong            => Self::LocaleTooLong,
        ClientInfoDecodeError::ZeroViewDist             => Self::ZeroViewDist,
        ClientInfoDecodeError::UnknownChatMode(v)       => Self::UnknownChatMode(v),
        ClientInfoDecodeError::UnknownMainHand(v)       => Self::UnknownMainHand(v),
        ClientInfoDecodeError::UnknownParticleStatus(v) => Self::UnknownParticleStatus(v)
    } }
}

impl From<IncompleteData> for C2SConfigPacketDecodeError {
    #[inline(always)]
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

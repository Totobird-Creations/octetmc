//! Information and settings about a client.


use crate::value::varint::VarIntDecodeError;
use crate::packet::decode::{ PacketPartDecode, DecodeBuf, DecodeBufHead, IncompleteData };
use crate::packet::decode::str::StringDecodeError;
use core::num::NonZeroU8;
use std::borrow::Cow;


mod chat_mode;
pub use chat_mode::*;

mod skin_parts;
pub use skin_parts::*;

mod main_hand;
pub use main_hand::*;

mod particle_status;
pub use particle_status::*;


/// Information and settings about a client.
///
/// This is sent when the player connects, or when settings are changed.
#[derive(Debug, Clone)]
pub struct ClientInfo<'l> {

    /// Currently selected player game language as a language code.
    pub locale                : Cow<'l, str>,

    /// Client-side render distance, in chunks.
    pub view_distance         : NonZeroU8,

    /// What kinds of messages the player will see in chat.
    ///
    /// <https://minecraft.wiki/w/Java_Edition_protocol/Chat#Client_chat_mode>
    pub chat_mode             : ClientChatMode,

    /// Whether the client will display chat messages in colour.
    pub chat_colours          : bool,

    /// Parts of the player's skin which are set to visible.
    pub skin_parts            : PlayerSkinParts,

    /// The player's selected main hand.
    pub main_hand             : PlayerMainHand,

    /// Enables filtering of text on signs and written book titles.
    ///
    /// Always `false` when mojauth is disabled.
    pub text_filtering        : bool,

    /// Servers usually list online players, this option should let you not show up in that list.
    pub allow_server_listings : bool,

    /// How many particles should be sent to the client.
    pub particle_status       : ClientParticleStatus

}

impl<'l> ClientInfo<'l> {

    /// Convert the inner parts of this `ClientInfo` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `ClientInfo<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> ClientInfo<'static> {
        ClientInfo {
            locale                : Cow::Owned(self.locale.into_owned()),
            view_distance         : self.view_distance,
            chat_mode             : self.chat_mode,
            chat_colours          : self.chat_colours,
            skin_parts            : self.skin_parts,
            main_hand             : self.main_hand,
            text_filtering        : self.text_filtering,
            allow_server_listings : self.allow_server_listings,
            particle_status       : self.particle_status
        }
    }

    /// Convert the inner parts of this `ClientInfo` to their owned counterparts.
    ///  Returns the newly created `ClientInfo<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> ClientInfo<'static> {
        ClientInfo {
            locale                : Cow::Owned((*self.locale).to_owned()),
            view_distance         : self.view_distance,
            chat_mode             : self.chat_mode,
            chat_colours          : self.chat_colours,
            skin_parts            : self.skin_parts,
            main_hand             : self.main_hand,
            text_filtering        : self.text_filtering,
            allow_server_listings : self.allow_server_listings,
            particle_status       : self.particle_status
        }
    }

}


impl PacketPartDecode for ClientInfo<'_> {

    type Output<'l> = ClientInfo<'l>;
    type Error<'l>  = ClientInfoDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        let locale       = buf.read_decode::<&str>(head)?;
        if (locale.len() > 16) { return Err(ClientInfoDecodeError::LocaleTooLong); }
        Ok(Self::Output {
            locale                : Cow::Borrowed(locale),
            view_distance         : NonZeroU8::new(buf.read(head)?).ok_or(ClientInfoDecodeError::ZeroViewDist)?,
            chat_mode             : buf.read_decode::<ClientChatMode>(head)?,
            chat_colours          : buf.read_decode::<bool>(head)?,
            skin_parts            : buf.read_decode::<PlayerSkinParts>(head)?,
            main_hand             : buf.read_decode::<PlayerMainHand>(head)?,
            text_filtering        : buf.read_decode::<bool>(head)?,
            allow_server_listings : buf.read_decode::<bool>(head)?,
            particle_status       : buf.read_decode::<ClientParticleStatus>(head)?
        })
    }

}


/// A `ClientInfo` failed to decode.
pub enum ClientInfoDecodeError {

    /// Not enough bytes were present.
    IncompleteData,

    /// The byte-encoded `VarInt` contained more than `MAX_BYTES` bytes.
    VarIntTooLong,

    /// A string contained invalid UTF-8.
    InvalidUtf8,

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

impl From<ClientInfoDecodeError> for Cow<'static, str> {
    fn from(value : ClientInfoDecodeError) -> Self { match (value) {
        ClientInfoDecodeError::IncompleteData           => IncompleteData.into(),
        ClientInfoDecodeError::VarIntTooLong            => VarIntDecodeError::TooLong.into(),
        ClientInfoDecodeError::InvalidUtf8              => StringDecodeError::InvalidUtf8.into(),
        ClientInfoDecodeError::LocaleTooLong            => Cow::Borrowed("locale too long"),
        ClientInfoDecodeError::ZeroViewDist             => Cow::Borrowed("view distance is zero"),
        ClientInfoDecodeError::UnknownChatMode(v)       => ChatModeDecodeError::UnknownChatMode(v).into(),
        ClientInfoDecodeError::UnknownMainHand(v)       => MainHandDecodeError::UnknownMainHand(v).into(),
        ClientInfoDecodeError::UnknownParticleStatus(v) => ParticleStatusDecodeError::UnknownParticleStatus(v).into()
    } }
}

impl From<VarIntDecodeError> for ClientInfoDecodeError {
    #[inline(always)]
    fn from(value : VarIntDecodeError) -> Self { match (value) {
        VarIntDecodeError::IncompleteData => Self::IncompleteData,
        VarIntDecodeError::TooLong        => Self::VarIntTooLong
    } }
}

impl From<StringDecodeError> for ClientInfoDecodeError {
    #[inline(always)]
    fn from(value : StringDecodeError) -> Self { match (value) {
        StringDecodeError::IncompleteData => Self::IncompleteData,
        StringDecodeError::VarIntTooLong  => Self::VarIntTooLong,
        StringDecodeError::InvalidUtf8    => Self::InvalidUtf8
    } }
}

impl From<ChatModeDecodeError> for ClientInfoDecodeError {
    #[inline(always)]
    fn from(value : ChatModeDecodeError) -> Self { match (value) {
        ChatModeDecodeError::IncompleteData     => Self::IncompleteData,
        ChatModeDecodeError::VarIntTooLong      => Self::VarIntTooLong,
        ChatModeDecodeError::UnknownChatMode(v) => Self::UnknownChatMode(v)
    } }
}

impl From<MainHandDecodeError> for ClientInfoDecodeError {
    #[inline(always)]
    fn from(value : MainHandDecodeError) -> Self { match (value) {
        MainHandDecodeError::IncompleteData     => Self::IncompleteData,
        MainHandDecodeError::VarIntTooLong      => Self::VarIntTooLong,
        MainHandDecodeError::UnknownMainHand(v) => Self::UnknownMainHand(v)
    } }
}

impl From<ParticleStatusDecodeError> for ClientInfoDecodeError {
    #[inline(always)]
    fn from(value : ParticleStatusDecodeError) -> Self { match (value) {
        ParticleStatusDecodeError::IncompleteData           => Self::IncompleteData,
        ParticleStatusDecodeError::VarIntTooLong            => Self::VarIntTooLong,
        ParticleStatusDecodeError::UnknownParticleStatus(v) => Self::UnknownParticleStatus(v)
    } }
}

impl From<IncompleteData> for ClientInfoDecodeError {
    #[inline(always)]
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

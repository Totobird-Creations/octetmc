//! Information and settings about a client.


use crate::value::varint::VarIntDecodeError;
use crate::packet::decode::{ PacketPartDecode, DecodeBuf, DecodeBufHead, IncompleteData };
use crate::packet::decode::str::StringDecodeError;
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
    pub locale                : &'l str,

    /// Client-side render distance, in chunks.
    pub view_distance         : u8,

    /// What kinds of messages the player will see in chat.
    ///
    /// https://minecraft.wiki/w/Java_Edition_protocol/Chat#Client_chat_mode
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


impl PacketPartDecode for ClientInfo<'_> {

    type Output<'l> = ClientInfo<'l>;
    type Error<'l>  = ClientInfoDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        let locale       = buf.read_decode::<&str>(head)?;
        if (locale.len() > 16) { return Err(ClientInfoDecodeError::LocaleTooLong); }
        Ok(Self::Output {
            locale,
            view_distance         : buf.read(head)?,
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

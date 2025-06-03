use crate::value::varint::{ VarInt, VarIntDecodeError };
use crate::packet::decode::{ PacketPartDecode, DecodeBuf, DecodeBufHead, IncompleteData };
use std::borrow::Cow;


/// <https://minecraft.wiki/w/Java_Edition_protocol/Chat#Client_chat_mode>
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ClientChatMode {

    /// All messages should be sent.
    Enabled,

    /// Player (disguised or not) chat messages should not be sent.
    ///  System messages should.
    CommandsOnly,

    /// No chat messages should be sent.
    ///  Overlay messages such as *"You may not rest now, the bed
    ///  is too far away"* should still be sent.
    Hidden

}


impl PacketPartDecode for ClientChatMode {

    type Output<'l> = ClientChatMode;
    type Error<'l>  = ChatModeDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(match (*buf.read_decode::<VarInt<u32>>(head)?) {
            0 => Self::Enabled,
            1 => Self::CommandsOnly,
            2 => Self::Hidden,
            v => { return Err(ChatModeDecodeError::UnknownChatMode(v)); }
        })
    }

}


/// A `ClientChatMode` failed to decode.
pub enum ChatModeDecodeError {

    /// Not enough bytes were present.
    IncompleteData,

    /// The byte-encoded `VarInt` contained more than `MAX_BYTES` bytes.
    VarIntTooLong,

    /// The client declared an unknown chat mode.
    UnknownChatMode(u32)

}

impl From<ChatModeDecodeError> for Cow<'static, str> {
    fn from(value : ChatModeDecodeError) -> Self { match (value) {
        ChatModeDecodeError::IncompleteData     => IncompleteData.into(),
        ChatModeDecodeError::VarIntTooLong      => VarIntDecodeError::TooLong.into(),
        ChatModeDecodeError::UnknownChatMode(_) => Cow::Borrowed("unknown chat mode")
    } }
}

impl From<VarIntDecodeError> for ChatModeDecodeError {
    #[inline(always)]
    fn from(value : VarIntDecodeError) -> Self { match (value) {
        VarIntDecodeError::IncompleteData => Self::IncompleteData,
        VarIntDecodeError::TooLong        => Self::VarIntTooLong
    } }
}

impl From<IncompleteData> for ChatModeDecodeError {
    #[inline(always)]
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

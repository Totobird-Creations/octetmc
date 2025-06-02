use crate::value::varint::{ VarInt, VarIntDecodeError };
use crate::packet::decode::{ PacketPartDecode, DecodeBuf, DecodeBufHead, IncompleteData };
use std::borrow::Cow;


/// The player's selected main hand.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum PlayerMainHand {

    /// The player has their main hand set to left.
    Left,

    /// The player has their main hand set to right.
    Right

}


impl PacketPartDecode for PlayerMainHand {

    type Output<'l> = PlayerMainHand;
    type Error<'l>  = MainHandDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(match (*buf.read_decode::<VarInt<u32>>(head)?) {
            0 => Self::Left,
            1 => Self::Right,
            v => { return Err(MainHandDecodeError::UnknownMainHand(v)); }
        })
    }

}


/// A `PlayerMainHand` failed to decode.
pub enum MainHandDecodeError {

    /// Not enough bytes were present.
    IncompleteData,

    /// The byte-encoded `VarInt` contained more than `MAX_BYTES` bytes.
    VarIntTooLong,

    /// The client declared an unknown main hand.
    UnknownMainHand(u32)

}

impl From<MainHandDecodeError> for Cow<'static, str> {
    fn from(value : MainHandDecodeError) -> Self { match (value) {
        MainHandDecodeError::IncompleteData     => IncompleteData.into(),
        MainHandDecodeError::VarIntTooLong      => VarIntDecodeError::TooLong.into(),
        MainHandDecodeError::UnknownMainHand(_) => Cow::Borrowed("unknown main hand")
    } }
}

impl From<VarIntDecodeError> for MainHandDecodeError {
    #[inline(always)]
    fn from(value : VarIntDecodeError) -> Self { match (value) {
        VarIntDecodeError::IncompleteData => Self::IncompleteData,
        VarIntDecodeError::TooLong        => Self::VarIntTooLong
    } }
}

impl From<IncompleteData> for MainHandDecodeError {
    #[inline(always)]
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

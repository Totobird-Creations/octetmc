use crate::value::varint::{ VarInt, VarIntDecodeError };
use crate::packet::decode::{ PacketPartDecode, DecodeBuf, DecodeBufHead, IncompleteData };
use std::borrow::Cow;


/// How many particles should be sent to the client.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ClientParticleStatus {

    /// All particles should be sent.
    All,

    /// A decreased number of particles should be sent.
    Decreased,

    /// Minimal particles should be sent.
    Minimal

}


impl PacketPartDecode for ClientParticleStatus {

    type Output<'l> = ClientParticleStatus;
    type Error<'l>  = ParticleStatusDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(match (*buf.read_decode::<VarInt<u32>>(head)?) {
            0 => Self::All,
            1 => Self::Decreased,
            2 => Self::Minimal,
            v => { return Err(ParticleStatusDecodeError::UnknownParticleStatus(v)); }
        })
    }

}


/// A `ClientParticleStatus` failed to decode.
pub enum ParticleStatusDecodeError {

    /// Not enough bytes were present.
    IncompleteData,

    /// The byte-encoded `VarInt` contained more than `MAX_BYTES` bytes.
    VarIntTooLong,

    /// The client declared an unknown main hand.
    UnknownParticleStatus(u32)

}

impl From<ParticleStatusDecodeError> for Cow<'static, str> {
    fn from(value : ParticleStatusDecodeError) -> Self { match (value) {
        ParticleStatusDecodeError::IncompleteData           => IncompleteData.into(),
        ParticleStatusDecodeError::VarIntTooLong            => VarIntDecodeError::TooLong.into(),
        ParticleStatusDecodeError::UnknownParticleStatus(_) => Cow::Borrowed("unknown particle status")
    } }
}

impl From<VarIntDecodeError> for ParticleStatusDecodeError {
    #[inline(always)]
    fn from(value : VarIntDecodeError) -> Self { match (value) {
        VarIntDecodeError::IncompleteData => Self::IncompleteData,
        VarIntDecodeError::TooLong        => Self::VarIntTooLong
    } }
}

impl From<IncompleteData> for ParticleStatusDecodeError {
    #[inline(always)]
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

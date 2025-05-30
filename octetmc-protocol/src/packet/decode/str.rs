use super::{ PacketPartDecode, DecodeBuf, DecodeBufHead, IncompleteData };
use crate::value::varint::{ VarInt, VarIntDecodeError };
use std::str;
use std::borrow::Cow;


impl PacketPartDecode for &str {

    type Output<'l> = &'l str;
    type Error<'l>  = StringDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead) -> Result<Self::Output<'l>, Self::Error<'l>> {
        let len = *buf.read_decode::<VarInt<u32>>(head)? as usize;
        Ok(str::from_utf8(buf.read_n(head, len)?)?)
    }

}


pub enum StringDecodeError {
    IncompleteData,
    VarIntTooLong,
    InvalidUtf8
}

impl From<StringDecodeError> for Cow<'static, str> {
    fn from(value : StringDecodeError) -> Self { match (value) {
        StringDecodeError::IncompleteData => IncompleteData.into(),
        StringDecodeError::VarIntTooLong  => VarIntDecodeError::TooLong.into(),
        StringDecodeError::InvalidUtf8    => Self::Borrowed("invalid utf8")
    } }
}

impl From<IncompleteData> for StringDecodeError {
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

impl From<VarIntDecodeError> for StringDecodeError {
    fn from(value : VarIntDecodeError) -> Self { match (value) {
        VarIntDecodeError::IncompleteData => Self::IncompleteData,
        VarIntDecodeError::TooLong        => Self::VarIntTooLong
    } }
}

impl From<str::Utf8Error> for StringDecodeError {
    fn from(_ : str::Utf8Error) -> Self { Self::InvalidUtf8 }
}

use super::{ Ident, IdentValidateError };
use crate::value::varint::{ VarInt, VarIntDecodeError };
use crate::packet::decode::{ PacketPartDecode, DecodeBuf, DecodeBufHead, IncompleteData };
use crate::packet::decode::str::StringDecodeError;
use crate::packet::encode::{ PacketPartEncode, EncodeBuf };
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};


impl Ser for Ident<'_> {
    #[inline]
    fn serialize<S>(&self, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
    where
        S : Serer
    { self.to_str().serialize(serer) }
}


impl PacketPartEncode for Ident<'_> {

    #[inline]
    fn predict_size(&self) -> usize {
        let (nspace, path,) = self.as_inner();
        VarInt::<u32>::MAX_BYTES
        + nspace.len()
        + 1
        + path.len()
    }

    #[inline]
    fn encode(&self, buf : &mut EncodeBuf) {
        let (nspace, path,) = self.as_inner();
        buf.encode_write(VarInt::<u32>::from((nspace.len() + 1 + path.len()) as u32));
        buf.write_n(nspace.as_bytes());
        buf.write(b':');
        buf.write_n(path.as_bytes());
    }

}



impl PacketPartDecode for Ident<'_> {

    type Output<'l> = Ident<'l>;
    type Error<'l>  = IdentDecodeError;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        let s = buf.read_decode::<&str>(head)?;
        Ok(Ident::parse_str_checked(s)?)
    }
}


/// An `Ident` failed to decode.
pub enum IdentDecodeError {

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

    /// The identifier has an empty path part.
    EmptyPathPart,

    /// The identifier path contains an invalid character.
    BadPathChar(char)

}

impl From<IdentDecodeError> for Cow<'static, str> {
    fn from(value : IdentDecodeError) -> Self { match (value) {
        IdentDecodeError::IncompleteData   => IncompleteData.into(),
        IdentDecodeError::VarIntTooLong    => VarIntDecodeError::TooLong.into(),
        IdentDecodeError::InvalidUtf8      => Self::Borrowed("invalid utf8"),
        IdentDecodeError::EmptyNspace      => Self::Borrowed("empty identifier namespace"),
        IdentDecodeError::BadNspaceChar(_) => Self::Borrowed("invalid character in identifier namespace"),
        IdentDecodeError::EmptyPathPart    => Self::Borrowed("empty identifier path part"),
        IdentDecodeError::BadPathChar(_)   => Self::Borrowed("invalid character in identifier path")
    } }
}

impl From<IncompleteData> for IdentDecodeError {
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

impl From<StringDecodeError> for IdentDecodeError {
    fn from(value : StringDecodeError) -> Self { match (value) {
        StringDecodeError::IncompleteData => Self::IncompleteData,
        StringDecodeError::VarIntTooLong  => Self::VarIntTooLong,
        StringDecodeError::InvalidUtf8    => Self::InvalidUtf8
    } }
}

impl From<IdentValidateError> for IdentDecodeError {
    fn from(value : IdentValidateError) -> Self { match (value) {
        IdentValidateError::EmptyNspace       => Self::EmptyNspace,
        IdentValidateError::BadNspaceChar(ch) => Self::BadNspaceChar(ch),
        IdentValidateError::EmptyPathPart     => Self::EmptyPathPart,
        IdentValidateError::BadPathChar(ch)   => Self::BadPathChar(ch)
    } }
}

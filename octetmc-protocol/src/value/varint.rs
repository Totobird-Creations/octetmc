//! Variable-sized integers.


use crate::packet::decode::{ DecodeBufHead, DecodeBuf, PacketPartDecode, IncompleteData };
use crate::packet::encode::{ EncodeBuf, PacketPartEncode };
use core::ops::{ Deref, DerefMut };
use std::borrow::Cow;

/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarInt
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:VarLong
pub struct VarInt<V>(V)
where
    V : VarIntType;

impl<V> VarInt<V>
where
    V : VarIntType
{

    /// The maximum number of bytes that a `VarInt` of this type can use.
    pub const MAX_BYTES : usize = V::MAX_BYTES;

    /// Encodes this `VarInt` into the given buffer.
    ///
    /// The returned slice may be shorter than the given buffer.
    #[inline(always)]
    pub fn encode_as_slice<'l>(&self, buf : &'l mut <V as VarIntType>::Buf) -> &'l [u8] {
        <V as VarIntType>::encode(&self, buf)
    }

}

impl<V> Deref for VarInt<V>
where
    V : VarIntType
{
    type Target = V;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<V> DerefMut for VarInt<V>
where
    V : VarIntType
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<V> PacketPartDecode for VarInt<V>
where
    V : VarIntType
{
    type Output<'l> = VarInt<V>;
    type Error<'l> = VarIntDecodeError;
    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead) -> Result<Self::Output<'l>, Self::Error<'l>> {
        <V as VarIntType>::decode(buf, head).map(VarInt)
    }
}

impl<V> PacketPartEncode for VarInt<V>
where
    V : VarIntType
{

    #[inline(always)]
    fn predict_size(&self) -> usize {
        <V as VarIntType>::MAX_BYTES
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        let mut array_buf = <<V as VarIntType>::Buf>::default();
        buf.write_n(<V as VarIntType>::encode(&self.0, &mut array_buf));
    }

}

impl<V> From<V> for VarInt<V>
where
    V : VarIntType
{
    #[inline(always)]
    fn from(value : V) -> Self { Self(value) }
}


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#VarInt_and_VarLong
pub const SEGMENT_BITS : u8 = 0x7F;
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#VarInt_and_VarLong
pub const CONTINUE_BIT : u8 = 0x80;


macro_rules! var_int_type_impl { ( $ty:ty $(,)? ) => {
    impl VarIntType for $ty {
        const MAX_BYTES : usize = size_of::<$ty>() + 1;
        type Buf = [u8; Self::MAX_BYTES];

        fn decode(buf : DecodeBuf<'_>, head : &mut DecodeBufHead)
            -> Result<Self, VarIntDecodeError>
        {
            const MAX_SHIFT : usize = size_of::<$ty>() * 8;
            let mut value = 0;
            let mut shift = 0;
            loop {
                let byte = buf.read(head)?;
                value |= ((byte & SEGMENT_BITS) as $ty) << shift;
                if ((byte & CONTINUE_BIT) == 0) { break; }
                shift += 7;
                if (shift > MAX_SHIFT) { return Err(VarIntDecodeError::TooLong); }
            }
            Ok(<$ty>::from_le(value))
        }

        fn encode<'l>(&self, buf : &'l mut Self::Buf) -> &'l mut [u8] {
            const SELF_SEGMENT_BITS : $ty = SEGMENT_BITS as $ty;
            const SELF_CONTINUE_BIT : $ty = CONTINUE_BIT as $ty;
            let mut i     = 0;
            let mut data  = (*self).to_le();
            loop {
                if ((data & (! SELF_SEGMENT_BITS)) == 0) {
                    *unsafe { buf.get_unchecked_mut(i) } = (data & 0xFF) as u8;
                    i += 1;
                    return &mut buf[0..i];
                }
                buf[i] = ((data & SELF_SEGMENT_BITS) | SELF_CONTINUE_BIT) as u8;
                i += 1;
                data = (data.cast_unsigned() >> 7).cast_signed();
            }
        }

    }
} }
macro_rules! var_int_type_remap_impl { ( $ty:ty => $from:ty $(,)? ) => {
    impl VarIntType for $ty {
        const MAX_BYTES : usize = <$from as VarIntType>::MAX_BYTES;
        type Buf = [u8; Self::MAX_BYTES];

        fn decode(buf : DecodeBuf<'_>, head : &mut DecodeBufHead)
            -> Result<Self, VarIntDecodeError>
        {
            <$from as VarIntType>::decode(buf, head).map(|value| value.cast_unsigned())
        }

        fn encode<'l>(&self, buf : &'l mut Self::Buf) -> &'l mut [u8] {
            <$from as VarIntType>::encode(&self.cast_signed(), buf)
        }

    }
} }


/// A `VarInt`-compatible type.
///
/// `u32`, `i32`, `u64`, and `i64` are compatible.
pub trait VarIntType : Sized {

    /// The maximum number of bytes that a `VarInt` of this type can use.
    const MAX_BYTES : usize;

    /// The buffer type passed into `encode`. This should almost always be `[u8; MAX_BYTES]`.
    type Buf : Default;

    /// Decode a `VarInt` of this type from a buffer.
    fn decode(buf : DecodeBuf<'_>, head : &mut DecodeBufHead) -> Result<Self, VarIntDecodeError>;

    /// Encodes this `VarInt` into the given buffer.
    ///
    /// The returned slice may be shorter than the given buffer.
    fn encode<'l>(&self, buf : &'l mut Self::Buf) -> &'l mut [u8];
}

var_int_type_impl!( i32 );
var_int_type_remap_impl!( u32 => i32 );

var_int_type_impl!( i64 );
var_int_type_remap_impl!( u64 => i64 );


/// A `VarInt` failed to decode.
pub enum VarIntDecodeError {

    /// Not enough bytes were present.
    IncompleteData,

    /// The byte-encoded `VarInt` contained more than `MAX_BYTES` bytes.
    TooLong

}

impl From<VarIntDecodeError> for Cow<'static, str> {
    fn from(value : VarIntDecodeError) -> Self { match (value) {
        VarIntDecodeError::IncompleteData => IncompleteData.into(),
        VarIntDecodeError::TooLong        => Self::Borrowed("varint too long")
    } }
}

impl From<IncompleteData> for VarIntDecodeError {
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

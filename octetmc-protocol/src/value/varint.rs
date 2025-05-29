use crate::packet::BufHead;
use crate::packet::decode::{ DecodeBuf, PacketPartDecode, IncompleteData };
use core::ops::{ Deref, DerefMut };


pub struct VarInt<V>(V)
where
    V : VarIntType;

impl<V> VarInt<V>
where
    V : VarIntType
{
    pub const MAX_BYTES : usize = V::MAX_BYTES;
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
    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut BufHead) -> Result<Self::Output<'l>, Self::Error<'l>> {
        <V as VarIntType>::decode(buf, head).map(VarInt)
    }
}


pub const SEGMENT_BITS : u8 = 0x7F;
pub const CONTINUE_BIT : u8 = 0x80;


macro_rules! var_int_type_impl { ( $ty:ty $(,)? ) => {
    impl VarIntType for $ty {
        const MAX_BYTES : usize = core::mem::size_of::<$ty>() + 1;
        fn decode(buf : DecodeBuf<'_>, head : &mut BufHead)
            -> Result<Self, VarIntDecodeError>
        {
            const MAX_SHIFT : usize = core::mem::size_of::<$ty>() * 8;
            let mut value = 0;
            let mut shift = 0;
            loop {
                let byte = buf.read(head)?;
                value |= ((byte & SEGMENT_BITS) as $ty) << shift;
                if ((byte & CONTINUE_BIT) == 0) { break; }
                shift += 7;
                if (shift > MAX_SHIFT) { return Err(VarIntDecodeError::TooLong); }
            }
            Ok(value)
        }
    }
} }
macro_rules! var_int_type_remap_impl { ( $ty:ty => $from:ty $(,)? ) => {
    impl VarIntType for $ty {
        const MAX_BYTES : usize = <$from as VarIntType>::MAX_BYTES;
        fn decode(buf : DecodeBuf<'_>, head : &mut BufHead)
            -> Result<Self, VarIntDecodeError>
        {
            <$from as VarIntType>::decode(buf, head).map(|value| value.cast_unsigned())
        }
    }
} }


pub trait VarIntType : Sized {
    const MAX_BYTES : usize;
    fn decode(buf : DecodeBuf<'_>, head : &mut BufHead) -> Result<Self, VarIntDecodeError>;
}

var_int_type_impl!( i32 );
var_int_type_remap_impl!( u32 => i32 );

var_int_type_impl!( i64 );
var_int_type_remap_impl!( u64 => i64 );


pub enum VarIntDecodeError {
    IncompleteData,
    TooLong
}

impl From<IncompleteData> for VarIntDecodeError {
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

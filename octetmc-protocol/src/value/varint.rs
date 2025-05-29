use crate::packet::decode::{ DecodeBuf, PacketPartDecode, DecodeError };
use core::ops::{ Deref, DerefMut };


pub struct VarInt<V>(V)
where
    V : VarIntType;

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
    fn decode(buf : &mut DecodeBuf<'_>) -> Result<Self, DecodeError> {
        <V as VarIntType>::decode(buf).map(VarInt)
    }
}


const SEGMENT_BITS : u8 = 0x7F;
const CONTINUE_BIT : u8 = 0x80;


macro_rules! var_int_type_impl { ( $ty:ty $(,)? ) => {
    impl VarIntType for $ty {
        fn decode(buf : &mut DecodeBuf<'_>) -> Result<Self, DecodeError> {
            const MAX_SHIFT : usize = core::mem::size_of::<$ty>() * 8;
            let mut value = 0;
            let mut shift = 0;
            loop {
                let byte = buf.read()?;
                value |= ((byte & SEGMENT_BITS) as $ty) << shift;
                if ((byte & CONTINUE_BIT) == 0) { break; }
                shift += 7;
                if (shift > MAX_SHIFT) { return Err(DecodeError::LongVarInt); }
            }
            Ok(value)
        }
    }
} }
macro_rules! var_int_type_remap_impl { ( $ty:ty => $from:ty $(,)? ) => {
    impl VarIntType for $ty {
        fn decode(buf : &mut DecodeBuf<'_>) -> Result<Self, DecodeError> {
            <$from as VarIntType>::decode(buf).map(|value| value.cast_unsigned())
        }
    }
} }


pub trait VarIntType : Sized {
    fn decode(buf : &mut DecodeBuf<'_>) -> Result<Self, DecodeError>;
}

var_int_type_impl!( i32 );
var_int_type_remap_impl!( u32 => i32 );

var_int_type_impl!( i64 );
var_int_type_remap_impl!( u64 => i64 );

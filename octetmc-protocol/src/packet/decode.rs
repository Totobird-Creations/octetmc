use super::{ PacketBound, PacketState, PacketBoundState };
use crate::value::varint::VarInt;
use std::str;


// /// Does **not** include the packet ID prefix.
pub trait PacketDecode : Sized + 'static
where
    (Self::Bound, Self::State,) : PacketBoundState
{
    type Bound : PacketBound;
    type State : PacketState;

    fn decode(buf : &mut DecodeBuf<'_>) -> Result<Self, DecodeError>;
}


pub trait PacketPartDecode : Sized {
    fn decode(buf : &mut DecodeBuf<'_>) -> Result<Self, DecodeError>;
}

impl<P> PacketPartDecode for P
where
    P                     : PacketDecode,
    (P::Bound, P::State,) : PacketBoundState
{
    fn decode(buf : &mut DecodeBuf<'_>) -> Result<Self, DecodeError> {
        <P as PacketDecode>::decode(buf)
    }
}

macro_rules! impl_packet_part_decode_for_num { ( $ty:ty $(,)? ) => {
    impl PacketPartDecode for $ty {
        fn decode(buf : &mut DecodeBuf<'_>) -> Result<Self, DecodeError> {
            Ok(<$ty>::from_be_bytes(buf.read_n_const()?))
        }
    }
} }
impl_packet_part_decode_for_num!(u8);
impl_packet_part_decode_for_num!(i8);
impl_packet_part_decode_for_num!(u16);
impl_packet_part_decode_for_num!(i16);
impl_packet_part_decode_for_num!(u32);
impl_packet_part_decode_for_num!(i32);
impl_packet_part_decode_for_num!(u64);
impl_packet_part_decode_for_num!(i64);
impl_packet_part_decode_for_num!(u128);
impl_packet_part_decode_for_num!(i128);
impl_packet_part_decode_for_num!(f32);
impl_packet_part_decode_for_num!(f64);

impl PacketPartDecode for String {
    fn decode(buf : &mut DecodeBuf<'_>) -> Result<Self, DecodeError> {
        let len = *buf.read_decode::<VarInt<u32>>()? as usize;
        Ok(str::from_utf8(buf.read_n(len)?)?.to_string())
    }
}


pub struct DecodeBuf<'l> {
    buf  : &'l [u8],
    head : usize
}

impl DecodeBuf<'_> {

    pub fn read(&mut self) -> Result<u8, DecodeError> {
        if let Some(&b) = self.buf.get(self.head) {
            self.head += 1;
            Ok(b)
        } else { Err(DecodeError::IncompleteData) }
    }

    pub fn read_n(&mut self, n : usize) -> Result<&[u8], DecodeError> {
        if let Some(bs) = self.buf.get(self.head..(self.head + n)) {
            self.head += n;
            Ok(bs)
        } else { Err(DecodeError::IncompleteData) }
    }

    pub fn read_n_const<const N : usize>(&mut self) -> Result<[u8; N], DecodeError> {
        if let Some(bs) = self.buf.get(self.head..(self.head + N)) {
            self.head += N;
            Ok(unsafe { bs.try_into().unwrap_unchecked() })
        } else { Err(DecodeError::IncompleteData) }
    }

    pub fn read_decode<T>(&mut self) -> Result<T, DecodeError>
    where
        T : PacketPartDecode
    {
        <T as PacketPartDecode>::decode(self)
    }

}


pub enum DecodeError {
    IncompleteData,
    LongVarInt,
    BadUtf8,
    UnknownIntention
}

impl From<str::Utf8Error> for DecodeError {
    fn from(_ : str::Utf8Error) -> Self { Self::BadUtf8 }
}

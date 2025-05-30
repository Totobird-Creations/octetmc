use super::{ PacketPartDecode, DecodeBuf, DecodeBufHead, IncompleteData };
use uuid::Uuid;


macro_rules! impl_packet_part_decode_for_num { ( $ty:ty $(,)? ) => {
    impl PacketPartDecode for $ty {
        type Output<'l> = $ty;
        type Error<'l>  = IncompleteData;
        fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead) -> Result<Self::Output<'l>, Self::Error<'l>> {
            Ok(<$ty>::from_be_bytes(buf.read_n_const(head)?))
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


impl PacketPartDecode for Uuid {
    type Output<'l> = Uuid;
    type Error<'l>  = IncompleteData;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>> {
            let msb = buf.read_decode::<u64>(head)?;
            let lsb = buf.read_decode::<u64>(head)?;
            Ok(Self::from_u64_pair(msb, lsb))
    }
}

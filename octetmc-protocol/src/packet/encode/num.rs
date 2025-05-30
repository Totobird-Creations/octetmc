use super::{ PacketPartEncode, EncodeBuf };


macro_rules! impl_packet_part_encode_for_num { ( $ty:ty $(,)? ) => {
    impl PacketPartEncode for $ty {

        fn encode(&self, buf : &mut EncodeBuf) {
            buf.write_n(&self.to_be_bytes());
        }

        #[inline(always)]
        fn predict_size(&self) -> usize {
            size_of::<$ty>()
        }

    }
} }
impl_packet_part_encode_for_num!(u8);
impl_packet_part_encode_for_num!(i8);
impl_packet_part_encode_for_num!(u16);
impl_packet_part_encode_for_num!(i16);
impl_packet_part_encode_for_num!(u32);
impl_packet_part_encode_for_num!(i32);
impl_packet_part_encode_for_num!(u64);
impl_packet_part_encode_for_num!(i64);
impl_packet_part_encode_for_num!(u128);
impl_packet_part_encode_for_num!(i128);
impl_packet_part_encode_for_num!(f32);
impl_packet_part_encode_for_num!(f64);

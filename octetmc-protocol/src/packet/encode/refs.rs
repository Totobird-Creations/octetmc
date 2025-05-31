use super::{ PacketPartEncode, EncodeBuf };
use std::borrow::Cow;


impl<T> PacketPartEncode for &T
where
    T : PacketPartEncode
{

    #[inline(always)]
    fn predict_size(&self) -> usize {
        <T as PacketPartEncode>::predict_size(*self)
    }

    #[inline(always)]
    fn encode(&self, buf : &mut EncodeBuf) {
        <T as PacketPartEncode>::encode(*self, buf)
    }

}


impl<T> PacketPartEncode for Cow<'_, T>
where
    T : PacketPartEncode + ToOwned + ?Sized
{

    #[inline(always)]
    fn predict_size(&self) -> usize {
        <T as PacketPartEncode>::predict_size(&**self)
    }

    #[inline(always)]
    fn encode(&self, buf : &mut EncodeBuf) {
        <T as PacketPartEncode>::encode(&**self, buf)
    }

}

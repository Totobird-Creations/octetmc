use super::{ PacketPartDecode, DecodeBuf, DecodeBufHead };


impl<T> PacketPartDecode for Option<T>
where
    T : PacketPartDecode
{

    type Output<'l> = Option<<T as PacketPartDecode>::Output<'l>>;
    type Error<'l>  = <T as PacketPartDecode>::Error<'l>;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>
    {
        Ok(if (buf.read(head)? != 0) {
            Some(<T as PacketPartDecode>::decode(buf, head)?)
        } else { None })
    }

}

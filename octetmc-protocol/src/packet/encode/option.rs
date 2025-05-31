use super::{ PacketPartEncode, EncodeBuf };


impl<T> PacketPartEncode for Option<T>
where
    T : PacketPartEncode
{

    #[inline]
    fn predict_size(&self) -> usize {
        self.as_ref().map(|v| v.predict_size() + 1).unwrap_or(1)
    }

    fn encode(&self, buf : &mut EncodeBuf) {
        if let Some(v) = self {
            buf.write(true);
            buf.encode_write(v);
        } else {
            buf.write(false);
        }
    }

}

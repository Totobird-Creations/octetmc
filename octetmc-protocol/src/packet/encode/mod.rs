//! Packet encoding utilities.


use super::PacketState;


mod num;

mod str;


/// Packet encoder, including packet ID.
pub trait PacketPrefixedEncode {
    /// The state in which this packet can be used.
    type State : PacketState;

    /// Predict the number of bytes it will take to encode this packet.
    ///
    /// Overestimate to avoid unnecessary reallocations.
    ///  If the size can not be predicted, return `0`.
    ///
    /// This method should take as little time as possible.
    ///  If the size value isn't already directly available from a method such as `len`, don't calculate it.
    fn predict_size(&self) -> usize;

    /// Encode the packet, including the packet's ID.
    fn encode_prefixed(&self, buf : &mut EncodeBuf);

}

impl<P> PacketPrefixedEncode for P
where
    P : PacketEncode
{
    type State = <P as PacketEncode>::State;

    fn predict_size(&self) -> usize {
        <P as PacketEncode>::predict_size(self)
    }

    fn encode_prefixed(&self, buf : &mut EncodeBuf) {
        buf.write(<P as PacketEncode>::PREFIX);
        <P as PacketEncode>::encode(self, buf)
    }

}

macro_rules! packet_encode_group { (
    type State = $state:ty;
    $( #[ $( $attr:tt )* ] )*
    $vis:vis enum $ident:ident $( < $( $lt:lifetime ),* $(,)? > )? {
        $(
            $( #[ $( $varattr:tt )* ] )*
            $varident:ident ( $varinner:ty $(,)? )
        ),* $(,)?
    }
) => {

    $( #[ $( $attr )* ] )*
    #[derive(Debug, Clone)]
    $vis enum $ident $( < $( $lt , )* > )? {
        $(
            $( #[ $( $varattr )* ] )*
            $varident ( $varinner ) ,
        )*
    }

        impl $( < $( $lt , )* > )? $crate::packet::encode::PacketPrefixedEncode for $ident $( < $( $lt , )* > )? {
            type State = $state;

            fn predict_size(&self) -> usize { match (self) {
                $(
                    Self::$varident(v) => {
                        <$varinner as $crate::packet::encode::PacketEncode>::predict_size(v)
                    },
                )*
                #[allow(unreachable_patterns)]
                _ => unreachable!()
            } }

            fn encode_prefixed(
                &self,
                #[allow(unused_variables)]
                buf : &mut $crate::packet::encode::EncodeBuf
            ) { match (self) {
                $(
                    Self::$varident(v) => {
                        buf.write(<$varinner as $crate::packet::encode::PacketEncode>::PREFIX);
                        <$varinner as $crate::packet::encode::PacketEncode>::encode(v, buf);
                    },
                )*
                #[allow(unreachable_patterns)]
                _ => unreachable!()
            } }

        }

} }
pub(crate) use packet_encode_group;


/// Packet encoder, excluding packet ID.
pub trait PacketEncode {
    /// The state in which this packet can be used.
    type State : PacketState;

    /// The ID of this packet.
    const PREFIX : u8;

    /// Predict the number of bytes it will take to encode this packet.
    ///
    /// Overestimate to avoid unnecessary reallocations.
    ///  If the size can not be predicted, return `0`.
    ///
    /// This method should take as little time as possible.
    ///  If the size value isn't already directly available from a method such as `len`, don't calculate it.
    fn predict_size(&self) -> usize;

    /// Encode the packet, excluding the packet's ID.
    fn encode(&self, buf : &mut EncodeBuf);

}


/// Packet part encoder.
pub trait PacketPartEncode {

    /// Encode this packet part.
    fn encode(&self, buf : &mut EncodeBuf);

    /// Predict the number of bytes it will take to encode this value.
    ///
    /// Overestimate to avoid unnecessary reallocations.
    ///  If the size can not be predicted, return `0`.
    ///
    /// This method should take as little time as possible.
    ///  If the size value isn't already directly available from a method such as `len`, don't calculate it.
    fn predict_size(&self) -> usize;

}


/// A buffer of bytes to encode and write a packet to.
#[derive(Default)]
pub struct EncodeBuf {
    buf : Vec<u8>
}

impl EncodeBuf {

    /// Return the number of bytes written to this buffer.
    #[inline]
    pub fn len(&self) -> usize { self.buf.len() }

    /// Return the number of bytes that this buffer has the space for without reallocating.
    #[inline]
    pub fn capacity(&self) -> usize { self.buf.capacity() }

    /// Reserve enough space in this buffer to add `n` more bytes.
    ///
    /// Implementors of [`PartPrefixedEncode`], [`PacketEncode`], and [`PacketPartEncode`]
    ///  should only call this if the size could not be predicted in `predict_size`.
    pub fn reserve(&mut self, additional : usize) {
        self.buf.reserve_exact(additional);
    }

    /// Return a slice of the bytes written to this buffer.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] { &self.buf }

    /// Write a single byte to this buffer.
    #[inline]
    pub fn write(&mut self, byte : u8) {
        self.buf.push(byte);
    }

    /// Write a slice of bytes to this buffer.
    #[inline]
    pub fn write_n(&mut self, bytes : &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    /// Encode and write a packet part to this buffer.
    #[inline(always)]
    pub fn encode_write<T>(&mut self, packet : &T)
    where
        T : PacketPartEncode + ?Sized
    {
        <T as PacketPartEncode>::encode(packet, self)
    }

}

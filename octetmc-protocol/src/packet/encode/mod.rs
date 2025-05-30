use super::PacketState;


mod num;

mod str;


pub trait PacketEncodeGroup {
    type State : PacketState;

    fn encode_prefixed(&self, buf : &mut EncodeBuf);

}

impl<P> PacketEncodeGroup for P
where
    P : PacketEncode
{
    type State = <P as PacketEncode>::State;

    fn encode_prefixed(&self, buf : &mut EncodeBuf) {
        buf.write(<P as PacketEncode>::PREFIX);
        <P as PacketEncode>::encode(self, buf)
    }

}

macro_rules! packet_encode_group { (
    type State = $state:ty;
    $vis:vis enum $ident:ident $( < $( $lt:lifetime ),* $(,)? > )? {
        $( $varident:ident ( $varinner:ty $(,)? ) ),* $(,)?
    }
) => {

    #[derive(Debug, Clone)]
    $vis enum $ident $( < $( $lt , )* > )? {
        $( $varident ( $varinner ) , )*
    }

        impl $( < $( $lt , )* > )? $crate::packet::encode::PacketEncodeGroup for $ident $( < $( $lt , )* > )? {
            type State = $state;

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


pub trait PacketEncode {
    type State : PacketState;

    const PREFIX : u8;

    /// Does **not** include the packet ID.
    fn encode(&self, buf : &mut EncodeBuf);

}


pub trait PacketPartEncode {

    fn encode(&self, buf : &mut EncodeBuf);

}


#[derive(Default)]
pub struct EncodeBuf {
    buf : Vec<u8>
}

impl EncodeBuf {

    #[inline]
    pub fn len(&self) -> usize { self.buf.len() }

    #[inline]
    pub fn reserve(&mut self, additional : usize) {
        self.buf.reserve_exact(additional);
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] { &self.buf }

    #[inline]
    pub fn write(&mut self, byte : u8) {
        self.buf.push(byte);
    }

    #[inline]
    pub fn write_n(&mut self, bytes : &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    #[inline]
    pub fn write_n_const<const N : usize>(&mut self, bytes : [u8; N]) {
        self.buf.extend_from_slice(&bytes);
    }

    #[inline(always)]
    pub fn encode_write<T>(&mut self, packet : &T)
    where
        T : PacketPartEncode + ?Sized
    {
        <T as PacketPartEncode>::encode(packet, self)
    }

}

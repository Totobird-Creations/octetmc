//! Packet decoding utilities.


use super::PacketState;
use std::borrow::Cow;


mod num;

pub mod str;

/// The maximum packet length allowed to be sent by the client.
pub const MAX_PACKET_LENGTH : usize = 2usize.pow(21) - 1;


/// Packet decoder, including packet ID.
pub trait PacketPrefixedDecode : Sized {
    /// The state in which this packet can be used.
    type State : PacketState;

    /// The type returned by `decode_prefixed` on success.
    type Output<'l>;
    /// The type returned by `decode_prefixed` on failure.
    type Error<'l>  : From<IncompleteData> + Into<Cow<'static, str>>;

    /// Decode the packet, including the packet's ID.
    fn decode_prefixed<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, UnknownPrefix<Self::Error<'l>>>;

}

impl<P> PacketPrefixedDecode for P
where
    P : PacketDecode
{
    type State = <P as PacketDecode>::State;

    type Output<'l> = <P as PacketDecode>::Output<'l>;
    type Error<'l > = <P as PacketDecode>::Error<'l>;

    fn decode_prefixed<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, UnknownPrefix<Self::Error<'l>>>
    {
        let prefix = buf.read(head).map_err(|e| UnknownPrefix::Error(Self::Error::from(e)))?;
        if (prefix != <P as PacketDecode>::PREFIX) {
            return Err(UnknownPrefix::UnknownPrefix(prefix));
        }
        <P as PacketDecode>::decode(buf, head).map_err(UnknownPrefix::Error)
    }

}

macro_rules! packet_decode_group { (
    type State = $state:ty;
    type Error<$errorlt:lifetime> = $error:ty;
    $( #[ $( $attr:tt )* ] )*
    $vis:vis enum $ident:ident $( < $lt:lifetime $(,)? > )? {
        $(
            $( #[ $( $varattr:tt )* ] )*
            $varident:ident ( $varinner:ty $(,)? )
        ),* $(,)?
    }
) => {

    $( #[ $( $attr )* ] )*
    #[derive(Debug, Clone)]
    $vis enum $ident $( < $lt , > )? {
        $(
            $( #[ $( $varattr )* ] )*
            $varident ( $varinner ) ,
        )*
    }

    impl $( < $lt , > )? $crate::packet::decode::PacketPrefixedDecode for $ident $( < $lt , > )? {
        type State = $state;

        $crate::packet::decode::packet_decode_group_output!{ $ident , $( $lt , )? }
        type Error<'errorlt> = $error;

        #[expect(non_snake_case)]
        fn decode_prefixed<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>(
            buf  : $crate::packet::decode::DecodeBuf<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>,
            head : &mut $crate::packet::decode::DecodeBufHead
        ) -> Result<
            Self::Output<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>,
            $crate::packet::decode::UnknownPrefix<Self::Error<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>>
        > {
            let prefix = buf.read(head).map_err(|e| $crate::packet::decode::UnknownPrefix::Error(Self::Error::from(e)))?;
            $( if (prefix == <$varinner as $crate::packet::decode::PacketDecode>::PREFIX) {
                return Ok($ident::$varident(
                    <$varinner as $crate::packet::decode::PacketDecode>::decode(buf, head)
                        .map_err(|e| $crate::packet::decode::UnknownPrefix::Error(
                            Into::<Self::Error<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>>::into(e)
                        ))?
                ));
            } )*
            Err($crate::packet::decode::UnknownPrefix::UnknownPrefix(prefix))
        }

    }

} }
macro_rules! packet_decode_group_output {
    ( $ident:ident , $lt:lifetime $(,)? ) => {
        #[expect(non_snake_case)]
        type Output<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME> = $ident<'__PACKET_GROUP_OUTPUT_ALTNAMED_LIFETIME>;
    };
    ( $ident:ident $(,)? ) => {
        #[expect(non_snake_case)]
        type Output<'__PACKET_GROUP_OUTPUT_UNUSED_LIFETIME> = $ident;
    };
}
pub(crate) use packet_decode_group;
pub(crate) use packet_decode_group_output;


/// Packet decoder, excluding packet ID.
pub trait PacketDecode : Sized {
    /// The state in which this packet can be used.
    type State : PacketState;

    /// The ID of this packet.
    const PREFIX : u8;
    /// The type returned by `decode` on success.
    type Output<'l>;
    /// The type returned by `decode` on failure.
    type Error<'l>  : From<IncompleteData> + Into<Cow<'static, str>>;

    /// Decode the packet, excluding the packet's ID.
    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>;

}


/// Packet part decoder.
pub trait PacketPartDecode : Sized {

    /// The type returned by `decode` on success.
    type Output<'l>;
    /// The type returned by `decode` on failure.
    type Error<'l>  : From<IncompleteData> + Into<Cow<'static, str>>;

    /// Decode this packet part.
    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>;

}


/// The head to read from a `DecodeBuf`.
///
/// This is separate from `DecodeBuf` due to lifetime conflicts.
#[derive(Default)]
pub struct DecodeBufHead {
    head : usize
}

impl DecodeBufHead {

    /// Returns the number of bytes that were consumed.
    #[inline(always)]
    pub fn consumed(&self) -> usize { self.head }

}


/// A buffer of bytes to read and decode a packet from.
#[derive(Clone, Copy, Debug)]
pub struct DecodeBuf<'l> {
    buf : &'l [u8]
}

impl<'l> DecodeBuf<'l> {

    /// Read a single byte from this buffer, incrementing `head`.
    pub fn read(&self, head : &mut DecodeBufHead)
        -> Result<u8, IncompleteData>
    {
        if let Some(&b) = self.buf.get(head.head) {
            head.head += 1;
            Ok(b)
        } else { Err(IncompleteData) }
    }

    /// Read `n` bytes from this buffer, incrementing `head`.
    pub fn read_n(&self, head : &mut DecodeBufHead, n : usize)
        -> Result<&'l [u8], IncompleteData>
    {
        let head_plus_n = head.head + n;
        if let Some(bs) = self.buf.get(head.head..head_plus_n) {
            head.head = head_plus_n;
            Ok(bs)
        } else { Err(IncompleteData) }
    }

    /// Read `N` bytes from this buffer, incrementing `head`.
    ///
    /// This is used by primitive types. Prefer `read_n` over
    ///  this, as it returns a reference to this buffer.
    pub fn read_n_const<const N : usize>(&self, head : &mut DecodeBufHead)
        -> Result<[u8; N], IncompleteData>
    {
        let head_plus_n = head.head + N;
        if let Some(bs) = self.buf.get(head.head..head_plus_n) {
            head.head = head_plus_n;
            Ok(unsafe { bs.try_into().unwrap_unchecked() })
        } else { Err(IncompleteData) }
    }

    /// Read and decode a packet part from this buffer.
    pub fn read_decode<T>(self, head : &mut DecodeBufHead)
        -> Result<<T as PacketPartDecode>::Output<'l>, <T as PacketPartDecode>::Error<'l>>
    where
        T : PacketPartDecode
    {
        <T as PacketPartDecode>::decode(self, head)
    }

}

impl<'l> From<&'l [u8]> for DecodeBuf<'l> {
    fn from(buf : &'l [u8]) -> Self {
        Self { buf }
    }
}


/// Not enough bytes were present.
pub struct IncompleteData;

impl From<IncompleteData> for Cow<'static, str> {
    fn from(_ : IncompleteData) -> Self { Self::Borrowed("incomplete data") }
}


/// Unknown prefix or other error.
pub enum UnknownPrefix<E> {

    /// An unknown or unexpected prefix was found.
    UnknownPrefix(u8),

    /// Some other error occurred.
    Error(E)

}

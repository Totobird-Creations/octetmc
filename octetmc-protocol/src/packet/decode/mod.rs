use super::PacketState;
use std::borrow::Cow;


mod num;

pub mod str;


pub const MAX_PACKET_LENGTH : usize = 2usize.pow(21) - 1;


pub trait PacketDecodeGroup : Sized {
    type State : PacketState;

    type Output<'l>;
    type Error<'l>  : From<IncompleteData> + Into<Cow<'static, str>>;

    fn decode_prefixed<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, UnknownPrefix<Self::Error<'l>>>;

}

impl<P> PacketDecodeGroup for P
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
    $vis:vis enum $ident:ident $( < $lt:lifetime $(,)? > )? {
        $( $varident:ident ( $varinner:ty $(,)? ) ),* $(,)?
    }
) => {

    #[derive(Debug, Clone)]
    $vis enum $ident $( < $lt , > )? {
        $( $varident ( $varinner ) , )*
    }

    impl $( < $lt , > )? $crate::packet::decode::PacketDecodeGroup for $ident $( < $lt , > )? {
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


pub trait PacketDecode : Sized {
    type State : PacketState;

    const PREFIX : u8;
    type Output<'l>;
    type Error<'l>  : From<IncompleteData> + Into<Cow<'static, str>>;

    /// Does **not** include the packet ID.
    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>;

}


pub trait PacketPartDecode : Sized {

    type Output<'l>;
    type Error<'l>  : From<IncompleteData> + Into<Cow<'static, str>>;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut DecodeBufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>;

}


#[derive(Default)]
pub struct DecodeBufHead {
    head : usize
}

impl DecodeBufHead {

    #[inline(always)]
    pub fn consumed(&self) -> usize { self.head }

}


#[derive(Clone, Copy, Debug)]
pub struct DecodeBuf<'l> {
    buf : &'l [u8]
}

impl<'l> DecodeBuf<'l> {

    pub fn read(&self, head : &mut DecodeBufHead)
        -> Result<u8, IncompleteData>
    {
        if let Some(&b) = self.buf.get(head.head) {
            head.head += 1;
            Ok(b)
        } else { Err(IncompleteData) }
    }

    pub fn read_n(&self, head : &mut DecodeBufHead, n : usize)
        -> Result<&'l [u8], IncompleteData>
    {
        let head_plus_n = head.head + n;
        if let Some(bs) = self.buf.get(head.head..head_plus_n) {
            head.head = head_plus_n;
            Ok(bs)
        } else { Err(IncompleteData) }
    }

    pub fn read_n_const<const N : usize>(&self, head : &mut DecodeBufHead)
        -> Result<[u8; N], IncompleteData>
    {
        let head_plus_n = head.head + N;
        if let Some(bs) = self.buf.get(head.head..head_plus_n) {
            head.head = head_plus_n;
            Ok(unsafe { bs.try_into().unwrap_unchecked() })
        } else { Err(IncompleteData) }
    }

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


pub struct IncompleteData;

impl From<IncompleteData> for Cow<'static, str> {
    fn from(_ : IncompleteData) -> Self { Self::Borrowed("incomplete data") }
}


pub enum UnknownPrefix<E> {
    UnknownPrefix(u8),
    Error(E)
}

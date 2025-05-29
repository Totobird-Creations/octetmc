use super::{ PacketBound, PacketState, PacketBoundState, BufHead };


mod num;

pub mod string;


pub const MAX_PACKET_LENGTH : usize = 2usize.pow(21) - 1;


pub trait PacketDecode : Sized
where
    (Self::Bound, Self::State,) : PacketBoundState
{
    type Bound : PacketBound;
    type State : PacketState;

    const PREFIX : u8;
    type Output<'l>;
    type Error<'l>  : From<IncompleteData>;

    /// Does **not** include the packet ID prefix.
    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut BufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>;

    fn decode_prefixed<'l>(buf : DecodeBuf<'l>, head : &mut BufHead)
        -> Result<Self::Output<'l>, UnknownPrefix<Self::Error<'l>>>
    {
        let prefix = buf.read(head).map_err(|e| UnknownPrefix::Error(Self::Error::from(e)))?;
        if (prefix != Self::PREFIX) {
            return Err(UnknownPrefix::UnknownPrefix(prefix));
        }
        Self::decode(buf, head).map_err(UnknownPrefix::Error)
    }
}


pub trait PacketPartDecode : Sized {

    type Output<'l>;
    type Error<'l>  : From<IncompleteData>;

    fn decode<'l>(buf : DecodeBuf<'l>, head : &mut BufHead)
        -> Result<Self::Output<'l>, Self::Error<'l>>;

}

// impl<P> PacketPartDecode for P
// where
//     P                     : PacketDecode,
//     (P::Bound, P::State,) : PacketBoundState
// {

//     type Output<'l> = <P as PacketDecode>::Output<'l>;
//     type Error<'l> = <P as PacketDecode>::Error<'l>;

//     fn decode<'l>(buf : DecodeBuf<'l>, head : &mut BufHead) -> Result<Self::Output<'l>, Self::Error<'l>> {
//         <P as PacketDecode>::decode(buf, head)
//     }

// }


#[derive(Clone, Copy)]
pub struct DecodeBuf<'l> {
    buf : &'l [u8]
}

impl<'l> DecodeBuf<'l> {

    pub fn read(&self, head : &mut BufHead)
        -> Result<u8, IncompleteData>
    {
        if let Some(&b) = self.buf.get(head.head) {
            head.head += 1;
            Ok(b)
        } else { Err(IncompleteData) }
    }

    pub fn read_n(&self, head : &mut BufHead, n : usize)
        -> Result<&'l [u8], IncompleteData>
    {
        let head_plus_n = head.head + n;
        if let Some(bs) = self.buf.get(head.head..head_plus_n) {
            head.head = head_plus_n;
            Ok(bs)
        } else { Err(IncompleteData) }
    }

    pub fn read_n_const<const N : usize>(&self, head : &mut BufHead)
        -> Result<[u8; N], IncompleteData>
    {
        let head_plus_n = head.head + N;
        if let Some(bs) = self.buf.get(head.head..head_plus_n) {
            head.head = head_plus_n;
            Ok(unsafe { bs.try_into().unwrap_unchecked() })
        } else { Err(IncompleteData) }
    }

    pub fn read_decode<T>(self, head : &mut BufHead)
        -> Result<<T as PacketPartDecode>::Output<'l>, <T as PacketPartDecode>::Error<'l>>
    where
        T : PacketPartDecode
    {
        <T as PacketPartDecode>::decode(self, head)
    }

}

impl<'l> From<&'l [u8]> for DecodeBuf<'l> {
    fn from(buf : &'l [u8]) -> Self { Self { buf } }
}


pub struct IncompleteData;


pub enum UnknownPrefix<E> {
    UnknownPrefix(u8),
    Error(E)
}

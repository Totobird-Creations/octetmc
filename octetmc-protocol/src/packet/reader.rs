use core::marker::PhantomData;


pub struct PacketReader<P : PacketDecode> {
    buf     : Vec<u8>,
    _marker : PhantomData<fn() -> P>
}

pub trait PacketDecode {}

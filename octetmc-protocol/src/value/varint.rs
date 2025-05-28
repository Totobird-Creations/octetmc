pub struct VarInt<V : VarIntInner> {
    value : V
}


const SEGMENT_BITS : u8 = 0x7F;
const CONTINUE_BIT : u8 = 0x80;

pub enum VarIntError {
    IncompleteData,

}

pub trait VarIntInner {
    const MAX_BYTES : usize;
}

impl VarIntInner for i32 {
    const MAX_BYTES : usize = 5;
}

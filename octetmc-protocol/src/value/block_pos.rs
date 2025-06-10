//! Positions of blocks in worlds.


use crate::packet::encode::{ PacketPartEncode, EncodeBuf };


/// A block position in a world.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BlockPos {

    /// X coordinate.
    ///
    /// Positive is east, negative is west.
    pub x : i32,

    /// Y coordinate.
    ///
    /// Positive is up, negative is down.
    pub y : i16,

    /// Z coordinate.
    ///
    /// Positive is south, negative is north.
    pub z : i32

}

impl PacketPartEncode for BlockPos {

    #[inline(always)]
    fn predict_size(&self) -> usize { 8 }

    #[inline]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write((((self.x as u64) & 0x3FFFFFF) << 38) | (((self.z as u64) & 0x3FFFFFF) << 12) | ((self.y as u64) & 0xFFF));
    }

}

//! Positions of blocks in worlds.


use super::character_pos::CharacterPos;
use crate::packet::encode::{ PacketPartEncode, EncodeBuf };
use saturating_cast::SaturatingCast;


/// A block position in a world.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct BlockPos {

    /// X coordinate.
    ///
    /// Positive is east, negative is west.
    pub x : i32,

    /// Y coordinate.
    ///
    /// Positive is up, negative is down.
    pub y : i32,

    /// Z coordinate.
    ///
    /// Positive is south, negative is north.
    pub z : i32

}

impl BlockPos {
    /// `BlockPos` with all coordinates set to `0`.
    pub const ZERO : Self = Self { x : 0, y : 0, z : 0 };
}

impl From<CharacterPos> for BlockPos {
    fn from(value : CharacterPos) -> Self { Self {
        x : (value.x as i64).saturating_cast::<i32>(),
        y : (value.y as i64).saturating_cast::<i32>(),
        z : (value.z as i64).saturating_cast::<i32>()
    } }
}


impl PacketPartEncode for BlockPos {

    #[inline(always)]
    fn predict_size(&self) -> usize { 8 }

    #[inline]
    fn encode(&self, buf : &mut EncodeBuf) {
        buf.encode_write((((self.x as u64) & 0x3FFFFFF) << 38) | (((self.z as u64) & 0x3FFFFFF) << 12) | ((self.y as u64) & 0xFFF));
    }

}

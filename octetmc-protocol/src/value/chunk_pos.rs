//! Positions of chunks in worlds.


use super::block_pos::BlockPos;
use super::character_pos::CharacterPos;


/// A chunk position in a world.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ChunkPos {

    /// X coordinate.
    ///
    /// Positive is east, negative is west.
    pub x : i32,

    /// Z coordinate.
    ///
    /// Positive is south, negative is north.
    pub z : i32

}

impl ChunkPos {
    /// ChunkPos with all coordinates set to `0`.
    pub const ZERO : Self = Self { x : 0, z : 0 };
}

impl From<BlockPos> for ChunkPos {
    fn from(value : BlockPos) -> Self { Self {
        x : value.x.div_euclid(16),
        z : value.z.div_euclid(16)
    } }
}

impl From<CharacterPos> for ChunkPos {
    #[inline]
    fn from(value : CharacterPos) -> Self {
        BlockPos::from(value).into()
    }
}

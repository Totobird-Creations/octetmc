//! Positions of chunks in worlds.


use crate::mapping_check;
use super::block_pos::BlockPos;
use super::chunk_section_pos::ChunkSectionPos;
use super::character_pos::CharacterPos;
use core::ops::Add;


mapping_check!("net.minecraft.world.level.ChunkPos", "bf0c345b39a1ac3acf7831f567c2a773f3f1954570eac72a9a93b4e55cb38ca3");


/// A chunk position in a world.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
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
    /// `ChunkPos` with all coordinates set to `0`.
    pub const ZERO : Self = Self { x : 0, z : 0 };
}

impl ChunkPos {

    /// Returns the [`ChunkSectionPos`] of section `y` in this chunk.
    #[inline(always)]
    pub const fn section(&self, y : u8) -> ChunkSectionPos {
        ChunkSectionPos { x : self.x, y, z : self.z }
    }

}

impl From<BlockPos> for ChunkPos {
    fn from(value : BlockPos) -> Self { Self {
        x : value.x.div_euclid(16),
        z : value.z.div_euclid(16)
    } }
}

impl From<ChunkSectionPos> for ChunkPos {
    fn from(value : ChunkSectionPos) -> Self { Self {
        x : value.x,
        z : value.z
    } }
}

impl From<CharacterPos> for ChunkPos {
    #[inline]
    fn from(value : CharacterPos) -> Self {
        BlockPos::from(value).into()
    }
}


impl Add for ChunkPos {
    type Output = Self;
    fn add(self, rhs : Self) -> Self::Output {
        Self { x : self.x + rhs.x, z : self.z + rhs.z }
    }
}

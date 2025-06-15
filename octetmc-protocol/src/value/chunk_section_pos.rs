//! Positions of chunks sections in worlds.


/// A chunk section position in a world.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
pub struct ChunkSectionPos {

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

impl ChunkSectionPos {
    /// `ChunkSectionPos` with all coordinates set to `0`.
    pub const ZERO : Self = Self { x : 0, y : 0, z : 0 };
}

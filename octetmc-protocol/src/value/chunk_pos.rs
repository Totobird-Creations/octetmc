//! Positions of chunks in worlds.


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

//! Block states.


/// A block state, including material and properties.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BlockState {
    id : u32
}

impl BlockState {

    /// Returns the integer ID of this block state.
    #[inline]
    pub fn id(&self) -> u32 { self.id }

    /// Create a new `BlockState` from an integer block state ID.
    #[inline]
    pub fn from_raw_id(id : u32) -> Self { Self { id } }

}

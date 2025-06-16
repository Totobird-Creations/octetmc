use super::ChunkSection;
use octetmc_protocol::value::block_state::BlockState;


impl ChunkSection {

    /// Returns a batch `ChunkSection` editor.
    pub fn edit(&mut self) -> ChunkSectionEdit<'_> {
        let blocks = self.into_array();
        ChunkSectionEdit { section : self, blocks }
    }

    /// Calls `f` with a batch editor for this `ChunkSection`. This is an alternative to [`Self::edit`],
    pub fn edit_scoped<F>(&mut self, f : F)
    where
        F : FnOnce(ChunkSectionEdit<'_>) -> ()
    {
        let edit = self.edit();
        f(edit);
    }

}


/// A [`ChunkSection`] editor.
pub struct ChunkSectionEdit<'l> {
    section : &'l mut ChunkSection,
    blocks  : [BlockState; 4096]
}


impl<'l> Drop for ChunkSectionEdit<'l> {
    #[inline]
    fn drop(&mut self) {
        *self.section = ChunkSection::from(self.blocks);
    }
}


impl ChunkSectionEdit<'_> {

    #[inline]
    #[track_caller]
    fn xyz_to_index([x, y, z,] : [u8; 3]) -> usize {
        debug_assert!(x < 16, "chunk section coordinate out of range");
        debug_assert!(y < 16, "chunk section coordinate out of range");
        debug_assert!(z < 16, "chunk section coordinate out of range");
        (256 * (y as usize)) + (16 * (x as usize)) + (z as usize)
    }


    /// Sets the block at index `i`. The axis order is Z-X-Y.
    ///
    /// # Panics
    /// Panics if `i` is not less than 4096.
    #[inline]
    #[track_caller]
    pub fn set_by_index(&mut self, i : usize, block : BlockState) {
        debug_assert!(i < 4096);
        self.blocks[i] = block;
    }

    /// Sets the block at `pos`.
    ///
    /// # Panics
    /// Panics if any coordinate in `pos` is not less than 16.
    #[inline(always)]
    #[track_caller]
    pub fn set(&mut self, pos : [u8; 3], block : BlockState) {
        self.set_by_index(Self::xyz_to_index(pos), block);
    }


    /// Gets the block at index `i`. The axis order is Z-X-Y.
    ///
    /// # Panics
    /// Panics if `i` is not less than 4096.
    #[inline]
    #[track_caller]
    pub fn get_by_index(&mut self, i : usize) -> BlockState {
        debug_assert!(i < 4096);
        self.blocks[i]
    }

    /// Gets the block at `pos`.
    ///
    /// # Panics
    /// Panics if any coordinate in `pos` is not less than 16.
    #[inline(always)]
    #[track_caller]
    pub fn get(&mut self, pos : [u8; 3]) -> BlockState {
        self.get_by_index(Self::xyz_to_index(pos))
    }

}

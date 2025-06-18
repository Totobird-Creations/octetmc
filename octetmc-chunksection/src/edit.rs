use super::{ ChunkSection, ChunkSectionDirty };
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

impl<'l> ChunkSectionEdit<'l> {

    pub unsafe fn from_raw(
        section : &'l mut ChunkSection,
        blocks  : [BlockState; 4096]
    ) -> Self { Self { section, blocks } }

}

impl<'l> Drop for ChunkSectionEdit<'l> {
    #[inline]
    fn drop(&mut self) {
        *self.section = ChunkSection::from(self.blocks);
    }
}

impl ChunkSectionEdit<'_> {

    /// Get the blocks in this section as a slice. The axis order is Z-X-Y.
    #[inline]
    pub fn as_slice(&self) -> &[BlockState; 4096] { &self.blocks }

    /// Get the blocks in this section as a mutable slice. The axis order is Z-X-Y.
    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [BlockState; 4096] { &mut self.blocks }

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


    /// Sets the block at index `i`. The axis order is Z-X-Y.
    ///
    /// # Panics
    /// Panics if `i` is not less than 4096.
    #[inline]
    #[track_caller]
    pub fn set_by_index<B>(&mut self, i : usize, block : B)
    where
        B : Into<BlockState>
    {
        debug_assert!(i < 4096);
        self.section.dirty.add(i as u16);
        self.blocks[i] = block.into();
    }

    /// Sets the block at `pos`.
    ///
    /// # Panics
    /// Panics if any coordinate in `pos` is not less than 16.
    #[inline(always)]
    #[track_caller]
    pub fn set<B>(&mut self, pos : [u8; 3], block : B)
    where
        B : Into<BlockState>
    { self.set_by_index(Self::xyz_to_index(pos), block); }

    /// Set every block to `block`.
    pub fn fill<B>(&mut self, block : B)
    where
        B : Into<BlockState>
    {
        self.section.dirty = ChunkSectionDirty::Many;
        self.blocks.fill(block.into());
    }

    /// Set every block in layer `y` to `block`.
    ///
    /// # Panics
    /// Panics if `y` is not less than 16.
    pub fn fill_layer<B>(&mut self, y : u8, block : B)
    where
        B : Into<BlockState>
    {
        debug_assert!(y < 16, "chunk section coordinate out of range");
        self.section.dirty = ChunkSectionDirty::Many;
        let i0 = 16*16 * (y as usize);
        let i1 = 16*16 * ((y as usize) + 1);
        self.blocks[i0..i1].fill(block.into());
    }


}

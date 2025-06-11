//! Automatic world generators.


use octetmc_protocol::value::chunk_pos::ChunkPos;
use octetmc_protocol::value::block_state::BlockState;


mod void;
pub use void::*;

mod superflat;
pub use superflat::*;


/// An automatic world generator.
pub trait WorldGenerator {

    /// Generate a chunk section.
    fn fill_section(&self, chunk : ChunkPos, section : u8, buf : &mut ChunkSectionBuf);

}


/// Part of a chunk section.
pub struct ChunkSectionBuf {
    blocks : [BlockState; 4096]
}

impl ChunkSectionBuf {

    /// Get a block in this chunk section.
    pub fn get(&mut self, x : u8, y : u8, z : u8) -> BlockState {
        assert!(x < 16);
        assert!(y < 16);
        assert!(z < 16);
        let i = ((y as usize) * 16 * 16) + ((x as usize) * 16) + (z as usize);
        self.blocks[i]
    }

    /// Set a block in this chunk section.
    pub fn set<B : Into<BlockState>>(&mut self, [x, y, z,] : [u8; 3], block : B) {
        assert!(x < 16);
        assert!(y < 16);
        assert!(z < 16);
        let i = ((y as usize) * 16 * 16) + ((x as usize) * 16) + (z as usize);
        self.blocks[i] = block.into();
    }

    /// Fill this entire chunk section with a block.
    pub fn fill<B : Into<BlockState>>(&mut self, block : B) {
        self.blocks.fill(block.into());
    }

    /// Fill a layer of this chunk section with a block.
    pub fn fill_layer<B : Into<BlockState>>(&mut self, y : u8, block : B) {
        assert!(y < 16);
        let i = (y as usize) * 16 * 16;
        let j = ((y + 1) as usize) * 16 * 16;
        unsafe { self.blocks.get_unchecked_mut(i..j) }.fill(block.into());
    }

}

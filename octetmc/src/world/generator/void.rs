use super::{ WorldGenerator, ChunkSectionBuf };
use octetmc_protocol::value::chunk_pos::ChunkPos;
use octetmc_protocol::registry::block::air::Air;


/// A [`WorldGenerator`] which places no blocks.
pub struct VoidGenerator;

impl WorldGenerator for VoidGenerator {

    #[inline]
    fn fill_section(&self, _chunk : ChunkPos, _section : u8, buf : &mut ChunkSectionBuf) {
        buf.fill(Air);
    }

}

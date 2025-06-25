use super::WorldGenerator;
use crate::player::PlayerId;
use crate::world::chunk::section::ChunkSectionEdit;
use octetmc_protocol::value::chunk_section_pos::ChunkSectionPos;
use octetmc_protocol::value::block::air::Air;


/// A [`WorldGenerator`] which places no blocks.
pub struct VoidGenerator;

impl WorldGenerator for VoidGenerator {

    #[inline]
    fn fill_section(&self, _player : PlayerId, _pos : ChunkSectionPos, mut edit : ChunkSectionEdit<'_>) {
        edit.fill(Air);
    }

}

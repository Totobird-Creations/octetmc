use super::WorldGenerator;
use crate::player::PlayerId;
use crate::world::chunk::section::ChunkSectionEdit;
use std::borrow::Cow;
use octetmc_protocol::value::chunk_section_pos::ChunkSectionPos;
use octetmc_protocol::value::block_state::BlockState;
use octetmc_protocol::value::block::air::Air;
use octetmc_protocol::value::block::bedrock::Bedrock;
use octetmc_protocol::value::block::dirt::Dirt;
use octetmc_protocol::value::block::grass_block::GrassBlock;


/// A [`WorldGenerator`] which places layers of blocks.
pub struct SuperflatGenerator {

    /// The layers in this world, starting from the bottom.
    pub layers : Cow<'static, [BlockState]>

}

impl Default for SuperflatGenerator {
    #[inline]
    fn default() -> Self { Self {
        layers : Cow::Borrowed(const { &[
            Bedrock.to_block_state(),
            Dirt.to_block_state(),
            Dirt.to_block_state(),
            (GrassBlock { snowy : false }).to_block_state()
        ] })
    } }
}

impl WorldGenerator for SuperflatGenerator {

    fn fill_section(&self, _player : PlayerId, pos : ChunkSectionPos, mut edit : ChunkSectionEdit<'_>) {
        let min_y = (pos.y as usize) * 16;
        for i in 0..16 {
            let y = min_y + i;
            edit.fill_layer(i as u8, self.layers.get(y).cloned().unwrap_or(Air.into()));
        }
    }

}

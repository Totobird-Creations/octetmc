//! Automatic world generators.


use crate::player::PlayerId;
use crate::world::chunk::ChunkTrackedEvent;
use crate::world::chunk::section::{ ChunkSectionBundle, ChunkSection, ChunkSectionEdit };
use octetmc_protocol::value::chunk_section_pos::ChunkSectionPos;
use octetmc_protocol::registry::block::air::Air;
use std::sync::{ Arc, Mutex };
use bevy_app::{ App, Plugin, Update };
use bevy_ecs::system::{ ParallelCommands, Res };
use bevy_ecs::resource::Resource;
use bevy_ecs::event::EventReader;


mod void;
pub use void::*;

mod superflat;
pub use superflat::*;


/// Bevy [`Plugin`] for automatically loading player chunks using a generator.
pub struct OctetAutoChunksPlugin {
    generator : Mutex<Option<Box<dyn WorldGenerator + Send + Sync>>>
}

impl OctetAutoChunksPlugin {

    /// Create a new `OctetAutoChunksPlugin` from a generator.
    #[inline]
    pub fn new<G>(generator : G) -> Self
    where
        G : WorldGenerator + Send + Sync + 'static
    { Self { generator : Mutex::new(Some(Box::new(generator))) } }

}

impl Default for OctetAutoChunksPlugin {
    #[inline(always)]
    fn default() -> Self { Self::new(VoidGenerator) }
}

impl Plugin for OctetAutoChunksPlugin {
    fn build(&self, app : &mut App) {
        app .insert_resource(AutoChunkGenerator(Arc::from(self.generator.lock().unwrap().take().unwrap())))
            .add_systems(Update, generate_chunks);
    }
}


/// An automatic world generator.
pub trait WorldGenerator {

    /// Generate a chunk section.
    fn fill_section(&self, player : PlayerId, pos : ChunkSectionPos, edit : ChunkSectionEdit<'_>);

}


#[derive(Resource)]
struct AutoChunkGenerator(Arc<dyn WorldGenerator + Send + Sync>);

fn generate_chunks(
        pcmds       : ParallelCommands,
        r_generator : Res<AutoChunkGenerator>,
    mut er_tracked  : EventReader<ChunkTrackedEvent>
) {
    er_tracked.par_read().for_each(|event| {
        pcmds.command_scope(|mut cmds| {
            let player    = event.player;
            let chunk_pos = event.pos;
            let generator = Arc::clone(&r_generator.0);
            cmds.spawn_batch((0..event.sections).map(move |y| {
                let section_pos = chunk_pos.section(y);
                ChunkSectionBundle {
                    player,
                    pos     : section_pos,
                    section : {
                        let mut section = ChunkSection::AIR;
                        let     edit    = unsafe { ChunkSectionEdit::from_raw(
                            &mut section,
                            [Air.to_block_state(); 4096]
                        ) };
                        generator.fill_section(player, section_pos, edit);
                        section
                    }
                }
            }));
        });
    });
}

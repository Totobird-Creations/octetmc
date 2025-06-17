//! World chunk data.


use super::{ MaxViewDistance, DEFAULT_VIEW_DISTANCE };
use crate::conn::ConnInPlay;
use crate::player::{ PlayerId, Player };
use crate::world::{ ChunkCentre, ViewDistance };
use crate::world::dimension::Dimension;
use crate::util::ecs::ParallelEventWriter;
use octetmc_protocol::value::chunk_pos::ChunkPos;
use core::mem::MaybeUninit;
use std::collections::BTreeMap;
use bevy_ecs::entity::Entity;
use bevy_ecs::component::Component;
use bevy_ecs::system::{ ParallelCommands, Res, ResMut, Query };
use bevy_ecs::query::With;
use bevy_ecs::resource::Resource;
use bevy_ecs::event::{ Event, EventWriter };


pub mod section;
use section::{ ChunkSectionId, ChunkSection };


/// A chunk section was unloaded.
#[derive(Event)]
pub struct ChunkSectionLoadEvent {

    /// The player the chunk will be tied to.
    pub player  : PlayerId,

    /// The position of the chunk.
    pub chunk   : ChunkPos,

    /// The chunk section index. `0` is bottom.
    pub section : u8

}


/// A chunk was unloaded.
#[derive(Event)]
pub struct ChunkUnloadEvent {

    /// The player the chunk was tied to.
    pub player   : PlayerId,

    /// The position of the chunk.
    pub chunk    : ChunkPos,

    /// IDs of the chunk sections that were despawned.
    pub sections : Box<[Option<ChunkSectionId>]>

}


#[derive(Component, Default)]
pub(crate) struct PlayerChunks {
    chunks : BTreeMap<ChunkPos, Box<[Option<ChunkSectionId>]>>
}



#[derive(Resource, Default)]
pub(super) struct ChunkLoadOrder(Vec<Vec<ChunkPos>>);

pub(super) fn cache_chunk_load_order(
    mut r_load_order    : ResMut<ChunkLoadOrder>,
        r_max_view_dist : Option<Res<MaxViewDistance>>
) {
    let v         = r_max_view_dist.map_or(DEFAULT_VIEW_DISTANCE, |v| **v).get() as usize;
    let cache_len = r_load_order.0.len().max(1);
    r_load_order.0.extend((cache_len..=v).map(|radius| {
        let     radius  = radius as i32;
        let mut offsets = ((-radius)..=radius).map(|x| ((-radius)..=radius).map(move |z| ChunkPos { x, z })).flatten().collect::<Vec<_>>();
        offsets.sort_by_key(|chunk| (chunk.x * chunk.x) + (chunk.z * chunk.z));
        offsets
    }));
}


pub(super) fn manage_chunks(
    mut q_players    : Query<(Entity, &mut PlayerChunks, &ChunkCentre, &ViewDistance, &Dimension<'static>), (With<Player>, With<ConnInPlay>,)>,
        r_load_order : Res<ChunkLoadOrder>,
        ew_load      : ParallelEventWriter<ChunkSectionLoadEvent>,
        ew_unload    : ParallelEventWriter<ChunkUnloadEvent>
) {
    q_players.par_iter_mut().for_each(|(entity, mut chunks, chunk_centre, view_dist, dimension,)| {
        if let Some(load_order) = r_load_order.0.get(view_dist.get() as usize).or(r_load_order.0.last()) {

            for i in (0..(chunks.chunks.len())).rev() {
                let chunk_to_unload = *(unsafe { chunks.chunks.keys().nth(i).unwrap_unchecked() });
                if (! load_order.contains(&chunk_to_unload)) {
                    // Chunk should not be loaded. Queue unload it.
                    ew_unload.write(ChunkUnloadEvent {
                        player   : PlayerId::from(entity),
                        chunk    : chunk_to_unload,
                        sections : unsafe { chunks.chunks.remove(&chunk_to_unload).unwrap_unchecked() }
                    });
                }
            }

            for offset in load_order {
                let chunk_to_load = **chunk_centre + *offset;
                if (! chunks.chunks.contains_key(&chunk_to_load)) {
                    // Chunk should be loaded. Queue load it.
                    let     height   = dimension.kind.height_sections.get();
                    let mut sections = Box::new_uninit_slice(height as usize);
                    sections.fill(MaybeUninit::new(None));
                    chunks.chunks.insert(chunk_to_load, unsafe { sections.assume_init() });
                    ew_load.write_batch((0..height).map(move |section| ChunkSectionLoadEvent {
                        player  : PlayerId::from(entity),
                        chunk   : chunk_to_load,
                        section
                    }));

                }
            }

        }
    });
}


pub(super) fn check_sections(

) {}

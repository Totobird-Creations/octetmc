//! World chunk data.


use super::{ MaxViewDistance, DEFAULT_VIEW_DISTANCE };
use crate::conn::ConnInPlay;
use crate::player::{ PlayerId, Player };
use crate::world::{ ChunkCentre, ViewDistance };
use crate::world::dimension::Dimension;
use crate::util::ecs::ParallelEventWriter;
use octetmc_protocol::value::chunk_pos::ChunkPos;
use octetmc_protocol::value::chunk_section_pos::ChunkSectionPos;
use core::mem::MaybeUninit;
use core::cmp::Ordering;
use std::collections::{ BTreeSet, BTreeMap };
use bevy_ecs::entity::Entity;
use bevy_ecs::component::Component;
use bevy_ecs::system::{ Commands, Res, ResMut, Query };
use bevy_ecs::query::With;
use bevy_ecs::resource::Resource;
use bevy_ecs::event::Event;


pub mod section;
use section::{ ChunkSectionId, ChunkSection };


/// A chunk section was unloaded.
#[derive(Event)]
pub struct ChunkTrackedEvent {

    /// The player the chunk will be tied to.
    pub player   : PlayerId,

    /// The position of the chunk section.
    pub pos      : ChunkPos,

    /// The number of sections in this chunk.
    pub sections : u8

}


/// A chunk was unloaded.
#[derive(Event)]
pub struct ChunkUntrackedEvent {

    /// The player the chunk was tied to.
    pub player   : PlayerId,

    /// The position of the chunk.
    pub chunk    : ChunkPos,

    /// IDs of the chunk sections that were despawned.
    pub sections : Box<[Option<ChunkSectionId>]>

}


#[derive(Component, Default)]
pub(crate) struct PlayerChunks {
    tracking : BTreeMap<ChunkPos, Box<[Option<ChunkSectionId>]>>,
    loaded   : BTreeSet<ChunkPos>
}



#[derive(Resource, Default)]
pub(super) struct ChunkLoadOrder(Vec<Vec<ChunkPos>>);

pub(super) fn cache_chunk_load_order(
    mut r_load_order    : ResMut<ChunkLoadOrder>,
        r_max_view_dist : Option<Res<MaxViewDistance>>
) {
    let v         = r_max_view_dist.map_or(DEFAULT_VIEW_DISTANCE, |v| **v).get() as usize;
    let cache_len = r_load_order.0.len() + 1;
    r_load_order.0.extend((cache_len..=v).map(|radius| {
        let     radius  = radius as i32;
        let mut offsets = ((-radius)..=radius).map(|x| ((-radius)..=radius).map(move |z| ChunkPos { x, z })).flatten().collect::<Vec<_>>();
        offsets.sort_by_key(|chunk| (chunk.x * chunk.x) + (chunk.z * chunk.z));
        offsets
    }));
}


pub(super) fn manage_chunk_tracking(
    mut q_players    : Query<(Entity, &mut PlayerChunks, &ChunkCentre, &ViewDistance, &Dimension<'static>), (With<Player>, With<ConnInPlay>,)>,
        r_load_order : Res<ChunkLoadOrder>,
        ew_load      : ParallelEventWriter<ChunkTrackedEvent>,
        ew_unload    : ParallelEventWriter<ChunkUntrackedEvent>
) {
    q_players.par_iter_mut().for_each(|(entity, mut chunks, chunk_centre, view_dist, dimension,)| {
        if let Some(load_order) = r_load_order.0.get(view_dist.get() as usize).or(r_load_order.0.last()) {

            for i in (0..(chunks.tracking.len())).rev() {
                let chunk_to_unload = *(unsafe { chunks.tracking.keys().nth(i).unwrap_unchecked() });
                if (! load_order.contains(&chunk_to_unload)) {
                    // Chunk should not be loaded. Queue untrack it.
                    ew_unload.write(ChunkUntrackedEvent {
                        player   : PlayerId::from(entity),
                        chunk    : chunk_to_unload,
                        sections : unsafe { chunks.tracking.remove(&chunk_to_unload).unwrap_unchecked() }
                    });
                }
            }

            for offset in load_order {
                let chunk_to_load = **chunk_centre + *offset;
                if (! chunks.tracking.contains_key(&chunk_to_load)) {
                    // Chunk should be loaded. Queue track it.
                    let     height   = dimension.kind.height_sections.get();
                    let mut sections = Box::new_uninit_slice(height as usize);
                    sections.fill(MaybeUninit::new(None));
                    chunks.tracking.insert(chunk_to_load, unsafe { sections.assume_init() });
                    ew_load.write(ChunkTrackedEvent {
                        player   : PlayerId::from(entity),
                        pos      : chunk_to_load,
                        sections : height
                    });

                }
            }

        }
    });
}


pub(super) fn check_tracking_sections(
    mut cmds       : Commands,
    mut q_sections : Query<(Entity, &PlayerId, &ChunkSectionPos, &mut ChunkSection)>,
    mut q_players  : Query<(&mut PlayerChunks,), (With<Player>, With<ConnInPlay>,)>
) {
    for (entity, player_id, section_pos, mut section,) in &mut q_sections {
        if let Ok((mut chunks,)) = q_players.get_mut(**player_id) {
            let chunk_pos = ChunkPos::from(*section_pos);
            if let Some(tracking_sections) = chunks.tracking.get_mut(&chunk_pos) {
                if let Some(tracking_section) = tracking_sections.get_mut(section_pos.y as usize) {
                    match (tracking_section) {
                        Some(tracking_section_id) => { match (entity.cmp(tracking_section_id)) {
                            Ordering::Less    => { },
                            Ordering::Equal   => { continue; },
                            Ordering::Greater => {
                                section.dirty_many();
                                *tracking_section_id = ChunkSectionId::from(entity);
                                println!("Track section {section_pos:?}");
                                continue;
                            },
                        } },
                        None => {
                            section.dirty_many();
                            *tracking_section = Some(ChunkSectionId::from(entity));
                            println!("Track section {section_pos:?}");
                            continue;
                        },
                    }
                }
            }
        }
        cmds.entity(entity).despawn();
    }
}


pub(super) fn check_untracking_sections(
    mut q_players  : Query<(Entity, &mut PlayerChunks,), (With<Player>, With<ConnInPlay>,)>,
        q_sections : Query<(&PlayerId, &ChunkSectionPos,), (With<ChunkSection>,)>
) {
    q_players.par_iter_mut().for_each(|(entity, mut chunks,)| {
        for (chunk_pos, tracked_sections,) in &mut chunks.tracking {
            for (y, tracked_section,) in tracked_sections.iter_mut().enumerate() {
                if let Some(tracked_section_id) = tracked_section {
                    let expected_section_pos = chunk_pos.section(y as u8);
                    if let Ok((section_player, section_pos,)) = q_sections.get(**tracked_section_id) {
                        if (**section_player == entity) {
                            if (*section_pos == expected_section_pos) {
                                continue;
                            }
                        }
                    }
                    *tracked_section = None;
                    println!("Untrack section {expected_section_pos:?}");
                }
            }
        }
    });
}

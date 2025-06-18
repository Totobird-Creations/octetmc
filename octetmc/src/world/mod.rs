//! World information and imports.


use crate::conn::out_message::ConnPeerOutMessage;
use crate::player::{ Player, ConnInPlay };
use crate::player::info::PlayerInfoUpdated;
use crate::util::dirty::Dirtyable;
use crate::util::macros::deref_single;
use octetmc_protocol::value::chunk_pos::ChunkPos;
use octetmc_protocol::value::character_pos::CharacterPos;
use octetmc_protocol::packet::play::s2c::S2CPlayPackets;
use octetmc_protocol::packet::play::s2c::set_chunk_cache_centre::SetChunkCacheCentreS2CPlayPacket;
use octetmc_protocol::packet::play::s2c::set_chunk_cache_radius::SetChunkCacheRadiusS2CPlayPacket;
use core::num::NonZeroU8;
use bevy_app::{ App, Plugin, Update };
use bevy_ecs::entity::Entity;
use bevy_ecs::component::Component;
use bevy_ecs::system::{ Query, ParallelCommands, Res };
use bevy_ecs::query::With;
use bevy_ecs::resource::Resource;
use bevy_ecs::event::EventReader;


pub mod dimension;

pub mod generator;

pub mod chunk;


pub(crate) const DEFAULT_VIEW_DISTANCE : NonZeroU8 = unsafe { NonZeroU8::new_unchecked(12) };


deref_single!{
    /// The server's maximum render distance.
    ///
    /// Vanilla clients support values up to 32. This can be set higher, but the client will clamp it.
    #[derive(Resource)]
    pub struct MaxViewDistance(NonZeroU8);
    From;
    Default { DEFAULT_VIEW_DISTANCE };
}


deref_single!{
    /// The centre of a client's loaded area.
    #[derive(Component)]
    pub struct ChunkCentre(ChunkPos);
    From;
    Dirtyable;
}


deref_single!{
    /// The client's render distance.
    ///
    /// This should not be higher than [`MaxViewDistance`].
    #[derive(Component)]
    pub struct ViewDistance(NonZeroU8);
    From;
    Dirtyable;
}


/// Bevy [`Plugin`] for handling worlds.
pub struct OctetWorldPlugin {

    /// The server's maximum render distance.
    pub max_view_distance : NonZeroU8

}

impl Default for OctetWorldPlugin {
    fn default() -> Self { Self {
        max_view_distance : DEFAULT_VIEW_DISTANCE
    } }
}

impl Plugin for OctetWorldPlugin {
    fn build(&self, app : &mut App) {
        app .add_event::<chunk::ChunkTrackedEvent>()
            .add_event::<chunk::ChunkUntrackedEvent>()
            .insert_resource(MaxViewDistance::from(self.max_view_distance))
            .insert_resource(chunk::ChunkLoadOrder::default())
            .add_systems(Update, chunk::cache_chunk_load_order)
            .add_systems(Update, update_view_distance)
            .add_systems(Update, send_view_distance)
            .add_systems(Update, update_chunk_centre)
            .add_systems(Update, send_chunk_centre)
            .add_systems(Update, chunk::manage_chunk_tracking)
            .add_systems(Update, chunk::check_tracking_sections)
            .add_systems(Update, chunk::check_untracking_sections);
    }
}


fn update_view_distance( // TODO: Add send_view_distance and update_chunk_centre.
        pcmds      : ParallelCommands,
        q_players  : Query<(Entity, &Player, Option<&ViewDistance>,)>,
        r_max_dist : Res<MaxViewDistance>,
    mut er_info    : EventReader<PlayerInfoUpdated>
) {
    er_info.par_read().for_each(|event| {
        if let Ok((entity, player, maybe_dist,)) = q_players.get(***event) {
            if let Some(info) = player.info() {
                let new_dist = info.view_distance.min(**r_max_dist);
                if (maybe_dist.is_none_or(|dist| new_dist != **dist)) {
                    let mut v = ViewDistance::from(new_dist);
                    v.mark_dirty();
                    pcmds.command_scope(|mut cmds| { cmds.entity(entity).insert(v); });
                }
            }
        }
    });
}

fn send_view_distance(
    mut q_players : Query<(&Player, &mut ViewDistance,), (With<ConnInPlay>,)>
) {
    q_players.par_iter_mut().for_each(|(player, mut view_dist,)| {
        if (view_dist.take_dirty()) {
            player.send_out_message(ConnPeerOutMessage::SendPlayPacket { packet : S2CPlayPackets::SetChunkCacheRadius(SetChunkCacheRadiusS2CPlayPacket {
                view_dist : **view_dist
            }) });
        }
    });
}


fn update_chunk_centre(
    pcmds     : ParallelCommands,
    q_players : Query<(Entity, &CharacterPos, Option<&ChunkCentre>,), (With<Player>,)>
) {
    q_players.par_iter().for_each(|(entity, pos, chunk_centre,)| {
        let new_chunk = ChunkPos::from(*pos);
        if (chunk_centre.is_none_or(|chunk| **chunk != new_chunk)) {
            let mut c = ChunkCentre::from(new_chunk);
            c.mark_dirty();
            pcmds.command_scope(|mut cmds| { cmds.entity(entity).insert(c); });
        }
    });
}

fn send_chunk_centre(
    mut q_players : Query<(&Player, &mut ChunkCentre,), (With<ConnInPlay>,)>
) {
    q_players.par_iter_mut().for_each(|(player, mut chunk_centre,)| {
        if (chunk_centre.take_dirty()) {
            player.send_out_message(ConnPeerOutMessage::SendPlayPacket { packet : S2CPlayPackets::SetChunkCacheCentre(SetChunkCacheCentreS2CPlayPacket {
                chunk : **chunk_centre
            }) });
        }
    });
}

//! World information and imports.


use crate::conn::out_message::ConnPeerOutMessage;
use crate::player::{ Player, ConnInPlay };
use crate::player::info::PlayerInfoUpdated;
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
use bevy_ecs::query::{ Changed, With };
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
}


deref_single!{
    /// The client's render distance.
    ///
    /// This should not be higher than [`MaxViewDistance`].
    #[derive(Component)]
    pub struct ViewDistance(NonZeroU8);
    From;
}


/// Bevy [`Plugin`] for handling worlds.
pub struct OctetWorldPlugin {
    max_view_distance : NonZeroU8
}

impl Default for OctetWorldPlugin {
    fn default() -> Self { Self {
        max_view_distance : DEFAULT_VIEW_DISTANCE
    } }
}

impl Plugin for OctetWorldPlugin {
    fn build(&self, app : &mut App) {
        app .insert_resource(MaxViewDistance::from(self.max_view_distance))
            .add_systems(Update, update_view_distances)
            .add_systems(Update, update_chunk_centre);
    }
}


fn update_view_distances(
        pcmds      : ParallelCommands,
        q_players  : Query<(Entity, &Player, Option<&ViewDistance>,), (With<ConnInPlay>,)>,
        r_max_dist : Res<MaxViewDistance>,
    mut er_info    : EventReader<PlayerInfoUpdated>
) {
    er_info.par_read().for_each(|event| {
        if let Ok((entity, player, maybe_dist,)) = q_players.get(***event) {
            if let Some(info) = player.info() {
                let new_dist = info.view_distance.min(**r_max_dist);
                if (maybe_dist.is_none_or(|dist| new_dist != **dist)) {
                    pcmds.command_scope(|mut cmds| { cmds.entity(entity).insert(ViewDistance::from(new_dist)); });
                    player.send_out_message(ConnPeerOutMessage::SendPlayPacket { packet : S2CPlayPackets::SetChunkCacheRadius(SetChunkCacheRadiusS2CPlayPacket {
                        view_dist : new_dist
                    }) });
                }
            }
        }
    });
}


fn update_chunk_centre(
    pcmds     : ParallelCommands,
    q_players : Query<(Entity, &Player, &CharacterPos, Option<&ChunkCentre>,), (Changed<CharacterPos>, With<ConnInPlay>,)>
) {
    q_players.par_iter().for_each(|(entity, player, pos, chunk_centre,)| {
        let new_chunk = ChunkPos::from(*pos);
        if (chunk_centre.is_none_or(|chunk| **chunk != new_chunk)) {
            pcmds.command_scope(|mut cmds| { cmds.entity(entity).insert(ChunkCentre::from(new_chunk)); });
            player.send_out_message(ConnPeerOutMessage::SendPlayPacket { packet : S2CPlayPackets::SetChunkCacheCentre(SetChunkCacheCentreS2CPlayPacket {
                chunk : new_chunk
            }) });
        }
    });
}

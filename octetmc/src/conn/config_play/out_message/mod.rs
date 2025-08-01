use super::{ ConfigPlay, ConnPeerComms, ConnPeerResult, config, play };
use crate::player::{ PlayerId, Player };
use crate::world::{ MaxViewDistance, DEFAULT_VIEW_DISTANCE };
use crate::world::dimension::Dimension;
use octetmc_protocol::value::ident::Ident;
use octetmc_protocol::value::game_mode::GameMode;
use octetmc_protocol::value::character_id::CharacterId;
use octetmc_protocol::value::character_pos::CharacterPos;
use octetmc_protocol::value::character_vel::CharacterVel;
use octetmc_protocol::value::entity_type::EntityType;
use octetmc_protocol::packet::config::s2c::registry_data::{ RegistryDataS2CConfigPacket, RegistryEntry };
use octetmc_protocol::packet::play::s2c::S2CPlayPackets;
use octetmc_protocol::packet::play::s2c::add_entity::AddEntityS2CPlayPacket;
use octetmc_protocol::packet::play::s2c::game_event::GameEventS2CPlayPacket;
use octetmc_protocol::packet::play::s2c::login::LoginS2CPlayPacket;
use octetmc_protocol::packet::play::s2c::respawn::RespawnS2CPlayPacket;
use std::borrow::Cow;
use bevy_defer::{ AsyncAccess, AsyncWorld };


pub(crate) enum ConnPeerOutMessage {
    Tick,

    SetRegistry {
        id      : Ident<'static>,
        entries : Vec<RegistryEntry<'static>>
    },

    Login {
        is_hardcore        : bool,
        dimension          : Dimension<'static>,
        reduced_debug_info : bool,
        respawn_screens    : bool,
        game_mode          : GameMode
    },

    SendPlayPacket {
        packet : S2CPlayPackets<'static>
    }

}


impl ConnPeerOutMessage {
    pub(super) async fn handle(self, player_id : PlayerId, comms : &mut ConnPeerComms) -> ConnPeerResult { match (self) {


        Self::Tick => {
            if (comms.is_logged_in()) {
                if let ConfigPlay::Config { active_ticks } = unsafe { comms.state_assume_config_play() } {
                    // 2 ticks is the maximum duration to read a Bevy event.
                    // All config events should be handled by now.
                    if (*active_ticks >= 1) {
                        unsafe { config::switch_to_play(player_id, comms) }.await?;
                    } else { *active_ticks += 1; }
                }
            }
            Ok(())
        },


        Self::SetRegistry { id, entries } => {
            unsafe { play::switch_to_config(player_id, comms) }.await?;
            comms.send_packet(&RegistryDataS2CConfigPacket {
                id, entries : Cow::Owned(entries),
            }).await?;
            Ok(())
        }


        Self::Login {
            is_hardcore,
            dimension,
            reduced_debug_info,
            respawn_screens,
            game_mode
        } => {
            if (comms.is_logged_in()) { panic!("player {} double-logged in", *player_id) }

            let view_distance = AsyncWorld.resource::<MaxViewDistance>().get(|r| **r).unwrap_or(DEFAULT_VIEW_DISTANCE).get();

            let Ok((uuid, pos, vel,)) = AsyncWorld.query::<(&Player, &CharacterPos, &CharacterVel,)>()
                .entity(*player_id).get(|(player, pos, vel,)| (player.profile().uuid, *pos, *vel,))
                else { panic!("player {} does not have a spawn `CharacterPos` and `CharacterVel`", *player_id) };

            unsafe { config::switch_to_play(player_id, comms) }.await?;

            comms.send_packet(&LoginS2CPlayPacket {
                entity_id            : 1,
                is_hardcore,
                dimensions           : Cow::Borrowed(&[dimension.kind.id.as_ref()]),
                max_players          : 1,
                view_distance,
                sim_distance         : view_distance,
                reduced_debug_info,
                respawn_screens,
                limited_crafting     : true,
                dimension_type       : 0,
                dimension            : dimension.kind.id.as_ref(),
                hashed_seed          : dimension.hashed_seed,
                game_mode,
                previous_game_mode   : None,
                is_debug             : false,
                is_superflat         : dimension.is_superflat,
                death_location       : None,
                portal_cooldown      : 0,
                sea_level            : dimension.sea_level,
                enforces_secure_chat : false
            }).await?;

            // TODO: PlayerInfoUpdateS2CPlayPacket

            comms.send_packet(&AddEntityS2CPlayPacket {
                eid      : CharacterId(0),
                uuid,
                kind     : EntityType::PLAYER,
                pos,
                vel,
                body_yaw : pos.yaw,
                data     : 0
            }).await?;

            comms.send_packet(&RespawnS2CPlayPacket {
                dimension_type     : 0,
                dimension          : dimension.kind.id.as_ref(),
                hashed_seed        : 0,
                game_mode,
                previous_game_mode : None,
                is_debug           : false,
                is_superflat       : dimension.is_superflat,
                death_location     : None,
                portal_cooldown    : 0,
                sea_level          : dimension.sea_level,
                keep_attributes    : false,
                keep_metadata      : false
            }).await?;

            _ = AsyncWorld.entity(*player_id).insert(dimension);

            // TODO: PlayerPositionS2CPlayPacket

            // TODO: wait for AcceptTeleportationC2SPlayPacket

            comms.send_packet(&GameEventS2CPlayPacket::WaitForChunks).await?;

            Ok(())
        },


        Self::SendPlayPacket { packet } => {
            unsafe { config::switch_to_play(player_id, comms) }.await?;
            comms.send_packet(&packet).await?;
            Ok(())
        }


    } }
}

use super::{ ConfigPlay, ConnPeerComms, ConnPeerResult, config };
use crate::player::PlayerId;
use crate::world::{ MaxViewDistance, DEFAULT_VIEW_DISTANCE };
use crate::world::dimension::Dimension;
use octetmc_protocol::value::game_mode::GameMode;
use octetmc_protocol::packet::play::s2c::login::LoginS2CPlayPacket;
use std::borrow::Cow;
use bevy_defer::{ AsyncAccess, AsyncWorld };


pub(crate) enum ConnPeerOutMessage {
    Tick,

    Login {
        is_hardcore        : bool,
        dimension          : Dimension<'static>,
        reduced_debug_info : bool,
        respawn_screens    : bool,
        limited_crafting   : bool,
        game_mode          : GameMode
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


        Self::Login {
            is_hardcore,
            dimension,
            reduced_debug_info,
            respawn_screens,
            limited_crafting,
            game_mode
        } => {
            if (comms.is_logged_in()) { panic!("Player double-logged in.") }
            unsafe { config::switch_to_play(player_id, comms) }.await?;

            // TODO: Send dimension registry.

            let view_distance = AsyncWorld.resource::<MaxViewDistance>().get(|r| **r).unwrap_or(DEFAULT_VIEW_DISTANCE).get();

            comms.send_packet(&LoginS2CPlayPacket {
                entity_id            : 1,
                is_hardcore,
                dimensions           : Cow::Borrowed(&[dimension.id.as_ref()]),
                max_players          : 0,
                view_distance,
                sim_distance         : view_distance,
                reduced_debug_info,
                respawn_screens,
                limited_crafting,
                dimension_type       : 0,
                dimension            : dimension.id.as_ref(),
                hashed_seed          : 0,
                game_mode,
                previous_game_mode   : None,
                is_debug             : false,
                is_superflat         : dimension.is_superflat,
                death_location       : None,
                portal_cooldown      : 0,
                sea_level            : dimension.sea_level,
                enforces_secure_chat : false
            }).await?;
            Ok(())
        }


    } }
}

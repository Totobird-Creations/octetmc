//! Player login request events.


use super::{ PlayerId, Player };
use crate::conn::out_message::ConnPeerOutMessage;
use crate::world::dimension::Dimension;
use octetmc_protocol::value::text::Text;
use octetmc_protocol::value::game_mode::GameMode;
use bevy_ecs::event::Event;


mod auto_login;
pub use auto_login::*;

mod kick_dupes;
pub use kick_dupes::*;


/// A player is trying to log in.
#[derive(Event)]
pub struct PlayerLoggingInEvent {

    /// The [`Entity`](bevy_ecs::entity::Entity) ID of the [`Player`](super::Player) who joined.
    pub player_id : PlayerId

}


/// Kick a player from the server.
#[derive(Event)]
pub struct KickPlayer {

    /// The [`Entity`](bevy_ecs::entity::Entity) ID of the [`Player`](super::Player) to kick.
    pub player_id : PlayerId,

    /// The message to display to the client.
    pub reason    : Text<'static, 'static>

}


impl Player {

    /// Logs the player in. This can be called **once** per player.
    ///
    /// ### Panics
    /// Panics if called a second time on the same player.
    pub fn login(&self,
        is_hardcore        : bool,
        dimension          : Dimension<'static>,
        reduced_debug_info : bool,
        respawn_screens    : bool,
        game_mode          : GameMode
    ) { self.send_out_message(ConnPeerOutMessage::Login {
        is_hardcore,
        dimension,
        reduced_debug_info,
        respawn_screens,
        game_mode
    }); }

}

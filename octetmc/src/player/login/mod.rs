//! Player login request events.


use super::PlayerId;
use octetmc_protocol::value::text::Text;
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

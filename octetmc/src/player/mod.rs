//! Player information and operations.


use crate::util::macros::{ CratePrivateNew, deref_single };
use core::ops::Deref;
use bevy_app::{ Plugin, App };
use bevy_ecs::entity::Entity;
use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;


mod profile;
pub use profile::*;

pub mod login;


/// A player connected to the server.
#[derive(Component)]
pub struct Player {
    pub(crate) profile : PlayerProfile
}

impl Player {

    /// Returns the player's profile.
    #[inline]
    pub fn profile(&self) -> &PlayerProfile { &self.profile }

}


/// The current number of players connected to the server.
///
/// The current player count is displayed in the server list
///  if this resource exists.
#[derive(Resource)]
pub struct PlayerCount(pub(crate) u32);

impl Deref for PlayerCount {
    type Target = u32;
    fn deref(&self) -> &Self::Target { &self.0 }
}


deref_single!{
    /// The maximum number of players allowed to be connected
    ///  to the server at the same time.
    ///
    /// The max player count is displayed in the server list.
    #[derive(Resource)]
    pub struct MaxPlayerCount(u32);
}


deref_single!{
    /// An [`Entity`] wrapper, intended for [`Player`]s.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct PlayerId(Entity);
    From;
}


/// Bevy [`Plugin`] for tracking players.
pub struct OctetPlayerPlugin {

    /// Whether to track the player count for the server status.
    pub track_player_count : bool,

    /// The maximum number of players allowed to be connected
    ///  to the server at the same time.
    pub max_player_count   : Option<u32>

}

impl Default for OctetPlayerPlugin {
    fn default() -> Self { Self {
        track_player_count : true,
        max_player_count   : Some(100)
    } }
}

impl Plugin for OctetPlayerPlugin {
    fn build(&self, app : &mut App) {
        app .add_event::<login::PlayerLoginEvent>()
            .add_event::<login::KickPlayer>();
        if (self.track_player_count) {
            app.insert_resource(PlayerCount(0));
        }
        if let Some(max_player_count) = self.max_player_count {
            app.insert_resource(MaxPlayerCount::crate_private_new(max_player_count));
        }
    }
}


/// Common imports.
pub mod prelude {
    pub use super::{
        Player,
        PlayerId,
        login::{
            PlayerLoginEvent,
            KickPlayer
        }
    };
}

//! Player information and operations.


use crate::util::macros::deref_single;
use core::ops::Deref;
use bevy_ecs::entity::Entity;
use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;


/// A player connected to the server.
#[derive(Component)]
pub struct Player;

/// The number of players connected to the server.
#[derive(Resource)]
pub struct PlayerCount(pub(crate) usize);
impl Deref for PlayerCount {
    type Target = usize;
    fn deref(&self) -> &Self::Target { &self.0 }
}


deref_single!{
    /// An [`Entity`] wrapper, intended for [`Player`]s.
    pub struct PlayerId(Entity);
}


/// Common imports.
pub mod prelude {
    pub use super::PlayerId;
}

//! Player information and operations.


use crate::util::macros::deref_single;
use bevy_ecs::entity::Entity;
use bevy_ecs::component::Component;


/// A player connected to the server.
#[derive(Component)]
pub struct Player;


deref_single!{
    /// An [`Entity`] wrapper, intended for [`Player`]s.
    pub struct PlayerId(Entity);
}


/// Common imports.
pub mod prelude {
    pub use super::PlayerId;
}

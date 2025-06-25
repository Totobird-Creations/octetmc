//! Networked IDs of player or non-player-characters in worlds.


use core::ops::Deref;


/// A networked character ID in a world.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
pub struct CharacterId(pub u32);

impl Deref for CharacterId {
    type Target = u32;
    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}

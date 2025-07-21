//! Velocity of player or non-player-characters in worlds.


/// A character position in a world.
///
/// Velocity is in blocks per second.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
pub struct CharacterVel {

    /// X velocity.
    ///
    /// Positive is east, negative is west.
    pub x     : f64,

    /// Y velocity.
    ///
    /// Positive is up, negative is down.
    pub y     : f64,

    /// Z velocity.
    ///
    /// Positive is south, negative is north.
    pub z     : f64

}

impl CharacterVel {
    /// `CharacterVel` with all coordinates set to `0.0`.
    pub const ZERO : Self = Self { x : 0.0, y : 0.0, z : 0.0 };
}

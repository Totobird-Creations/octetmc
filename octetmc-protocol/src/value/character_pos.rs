//! Positions of player or non-player-characters in worlds.


/// A character position in a world.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
pub struct CharacterPos {

    /// X coordinate.
    ///
    /// Positive is east, negative is west.
    pub x     : f64,

    /// Y coordinate.
    ///
    /// Positive is up, negative is down.
    pub y     : f64,

    /// Z coordinate.
    ///
    /// Positive is south, negative is north.
    pub z     : f64,

    /// Rotation pitch.
    ///
    /// Rotation is in radians.
    /// `-PI/2.0` is up.
    /// `PI/2.0` is down.
    pub pitch : f64,

    /// Rotation yaw.
    ///
    /// Rotation is in radians.
    /// `0.0` is positive z (south),
    /// `PI/2.0` is negative x (west).
    /// `PI` is negative z (north).
    /// `-PI/2.0` is positive x (east).
    pub yaw   : f64

}

impl CharacterPos {
    /// CharacterPos with all coordinates set to `0.0`.
    pub const ZERO : Self = Self { x : 0.0, y : 0.0, z : 0.0, pitch : 0.0, yaw : 0.0 };
}

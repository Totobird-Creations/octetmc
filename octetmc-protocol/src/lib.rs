#![forbid(missing_docs)]
//! # OctetMC (Protocol)
//! TODO: Top-level documentation


use semver::Version;


pub mod value;

pub mod packet;


/// Game versions that this library currently supports.
pub const GAME_VERSIONS : &[Version] = &[
    Version::new(1, 21, 5)
];

/// The latest game version that this library currently supports.
pub const LATEST_GAME_VERSION : &Version = &Version::new(1, 21, 5);

/// The protocol version that this library currently supports.
pub const PROTOCOL_VERSION : u32 = 770;

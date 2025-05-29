use semver::Version;


pub mod value;

pub mod packet;


pub const GAME_VERSIONS    : &[Version] = &[
    Version::new(1, 21, 5)
];
pub const PROTOCOL_VERSION : u32 = 770;

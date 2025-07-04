#![forbid(missing_docs)]
#![cfg_attr(doc, feature(doc_cfg))]
//! # OctetMC (Protocol)
//! TODO: Top-level documentation
//!
//!
//! ### Concepts
//!
//! #### Character
//! "`Character`s" are what Minecraft calls entities. octetmc-protocol calls them characters because "Entity" is a commonly used name in ECS libraries.
//!  Players, mobs, markers, displays, etc are all characters.
//!
//! ---
//!
//! Some documentation is copied from the [`Minecraft Wiki`](https://minecraft.wiki/w/Java_Edition_protocol), (formerly [wiki.vg](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge)).
//!
//! [wiki.vg](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge) is licensed under [Creative Commons Attribution-ShareAlike 3.0](http://creativecommons.org/licenses/by-sa/3.0/)
//!
//! The [Minecraft Wiki](https://minecraft.wiki/) is licensed under [Creative Commons Attribution-NonCommercial-ShareAlike 3.0](https://creativecommons.org/licenses/by-nc-sa/3.0/).


use semver::Version;


pub mod value;

pub mod packet;

pub mod registry;

include!(".generated/mappings.rs");


/// The game version that this library currently supports.
pub const GAME_VERSION : &Version = &Version::new(1, 21, 6);
/// The game version that this library currently supports (as a string).
pub const GAME_VERSION_STR : &str = "1.21.6";
/// The protocol version that this library currently supports.
pub const PROTOCOL_VERSION : u32 = 771;

include!(".generated/version.rs");


trait Sealed { }

#![forbid(missing_docs)]
//! # OctetMC
//! TODO: Top-level documentation


#![feature(
    // Language
    decl_macro
)]


use bevy_app::plugin_group;


pub mod conn;

pub mod server;

pub mod player;

pub mod world;


pub(crate) mod macros;


plugin_group! {
    #[derive(Debug)]
    pub struct OctetDefaultPlugins {
        conn:::OctetConnPlugin
    }
}

/// Common imports.
pub mod prelude {
    pub use super::OctetDefaultPlugins;
    pub use super::server::prelude::*;
    pub use super::player::prelude::*;

    /// Returns the [`Default`] value of `T`.
    pub fn default<T : Default>() -> T {
        <T as Default>::default()
    }
}

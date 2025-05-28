#![forbid(missing_docs)]
//! # OctetMC
//! TODO: Top-level documentation


#![feature(
    // Language
    decl_macro
)]


use bevy_app::{ plugin_group, Plugin, App };
use bevy_defer::AsyncPlugin;


pub mod conn;

pub mod server;

pub mod player;

pub mod world;


pub(crate) mod macros;


#[derive(Default)]
pub struct DefaultAsyncPlugin;
impl Plugin for DefaultAsyncPlugin {
    fn build(&self, app : &mut App) {
        app.add_plugins(AsyncPlugin::default_settings());
    }
}

plugin_group! {
    #[derive(Debug)]
    pub struct OctetDefaultPlugins {
        bevy_app:::ScheduleRunnerPlugin,
        :DefaultAsyncPlugin,
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

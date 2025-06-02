#![forbid(missing_docs)]
//! # OctetMC
//! TODO: Top-level documentation


use bevy_app::{ plugin_group, Plugin, App };
use bevy_defer::AsyncPlugin;

pub use octetmc_protocol as protocol;


pub mod conn;

pub mod server;

pub mod player;

pub mod world;


pub mod util;


/// A wrapper for [`AsyncPlugin`] which provides a [`Default`] implementation.
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
        bevy_time:::TimePlugin,
        bevy_diagnostic:::DiagnosticsPlugin,
        bevy_diagnostic:::FrameCountPlugin,
        :DefaultAsyncPlugin,
        conn:::OctetConnPlugin,
        player:::OctetPlayerPlugin,
        player::login:::PlayerKickDupesPlugin
    }
}

/// Common imports.
pub mod prelude {

    /// The bevy libraries that octectmc uses.
    ///
    /// https://docs.rs/bevy/latest/bevy
    pub mod bevy {
        pub use bevy_app as app;
        pub use bevy_ecs as ecs;
        pub use bevy_time as time;
        pub use bevy_diagnostic as diagnostic;
        pub use bevy_defer as defer;
        /// Common imports.
        pub mod prelude {
            pub use super::app::prelude::*;
            pub use super::ecs::prelude::*;
            pub use super::time::prelude::*;
        }
    }

    pub use super::protocol;
    pub use super::OctetDefaultPlugins;
    pub use super::server::prelude::*;
    pub use super::player::prelude::*;

    /// Returns the [`Default`] value of `T`.
    pub fn default<T : Default>() -> T {
        <T as Default>::default()
    }
}

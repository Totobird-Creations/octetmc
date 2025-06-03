#![forbid(missing_docs)]
#![cfg_attr(doc, feature(doc_cfg))]
//! # OctetMC
//! TODO: Top-level documentation
//!
//!
//! ---
//!
//! Some documentation is copied from the [`Minecraft Wiki`](https://minecraft.wiki/w/Java_Edition_protocol), (formerly [wiki.vg](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge)).
//!
//! [wiki.vg](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge) is licensed under [Creative Commons Attribution-ShareAlike 3.0](http://creativecommons.org/licenses/by-sa/3.0/)
//!
//! The [Minecraft Wiki](https://minecraft.wiki/) is licensed under [Creative Commons Attribution-NonCommercial-ShareAlike 3.0](https://creativecommons.org/licenses/by-nc-sa/3.0/).


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
        player::login:::PlayerAutoLoginPlugin,
        player::login:::PlayerKickDupesPlugin
    }
}

/// Common imports.
pub mod prelude {

    /// The bevy libraries that octectmc uses.
    ///
    /// <https://docs.rs/bevy/latest/bevy/>
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

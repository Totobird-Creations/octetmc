use octetmc::prelude::*;
use octetmc::conn::OctetConnPlugin;
use core::time::Duration;
use bevy_app::{ App, PluginGroup };


pub fn main() {
    App::new()
        .add_plugins(OctetDefaultPlugins.build()
            .set(OctetConnPlugin {
                mojauth_enabled : false,
                ..default()
            })
        )
        .run();
}

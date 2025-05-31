use octetmc::prelude::*;
use octetmc::conn::OctetConnPlugin;
use core::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };
use bevy_app::{ App, PluginGroup };


const LISTEN_ADDRS : &[SocketAddr] = &[
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 25580))
];


fn main() {
    App::new()
        .add_plugins(OctetDefaultPlugins.build()
            .set(OctetConnPlugin {
                listen_addrs    : LISTEN_ADDRS.into(),
                mojauth_enabled : true,
                ..default()
            })
        )
        .run();
}

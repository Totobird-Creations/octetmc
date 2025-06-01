use octetmc::prelude::*;
use octetmc::conn::OctetConnPlugin;
use core::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };
use bevy_app::{ App, PluginGroup, Update };
use bevy_ecs::system::Query;
use bevy_ecs::event::EventReader;


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
        .add_systems(Update, log_joins)
        .run();
}


fn log_joins(
    mut er_join   : EventReader<PlayerLoginEvent>,
        q_players : Query<(&Player,)>
) {
    for PlayerLoginEvent { player_id } in er_join.read() {
        if let Ok((player,)) = q_players.get(**player_id) {
            let profile = player.profile();
            println!("{} ({}) joined", profile.name, profile.uuid);
        }
    }
}

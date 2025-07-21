use octetmc::prelude::*;
use octetmc::server::ServerBrand;
use octetmc::conn::OctetConnPlugin;
use octetmc::world::OctetWorldPlugin;
use octetmc::world::generator::{ OctetAutoChunksPlugin, SuperflatGenerator };
use core::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };
use core::num::NonZeroU8;
use std::borrow::Cow;
use bevy_app::{ App, PluginGroup, Update };
use bevy_ecs::system::Query;
use bevy_ecs::event::EventReader;


const LISTEN_ADDRS : &[SocketAddr] = &[
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 25580))
];


fn main() {
    println!("starting");
    App::new()
        .add_plugins(OctetDefaultPlugins.build()
            .set(OctetConnPlugin {
                listen_addrs    : LISTEN_ADDRS.into(),
                mojauth_enabled : false,
                ..default()
            })
            .set(OctetWorldPlugin {
                max_view_distance : NonZeroU8::new(32).unwrap()
            })
            .set(OctetAutoChunksPlugin::new(SuperflatGenerator::default()))
        )
        .insert_resource(ServerBrand::from(Cow::Borrowed("OctetMC Example: Superflat")))
        .add_systems(Update, log_joins)
        .run();
}


fn log_joins(
    mut er_join   : EventReader<PlayerLoggingInEvent>,
        q_players : Query<(&Player,)>
) {
    for PlayerLoggingInEvent { player_id } in er_join.read() {
        if let Ok((player,)) = q_players.get(**player_id) {
            let profile = player.profile();
            println!("{} ({}) joined", profile.name, profile.uuid);
        }
    }
}

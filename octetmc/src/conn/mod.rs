//! Connection listener and client manager.


use crate::util::Never;
use core::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };
use std::borrow::Cow;
use std::io;
use smol::net::TcpListener;
use bevy_app::{ App, Plugin, Startup };
use bevy_ecs::component::Component;
use bevy_ecs::system::{ Commands, ResMut };
use bevy_ecs::resource::Resource;
use bevy_defer::{ AsyncWorld, Task, AsyncCommandsExtension };


pub mod error;

mod state;

mod comms;

mod handshake;

mod status;

mod login;

mod config_play;
pub(crate) use config_play::{ out_message, in_message };
pub use config_play::{ ConnInConfig, ConnInPlay };


/// Enables the connection listener and client manager on install.
#[derive(Clone)]
pub struct OctetConnPlugin {

    /// Addresses to listen on.
    ///
    /// The default port the game uses is `25565`.
    pub listen_addrs       : Cow<'static, [SocketAddr]>,

    /// How large packets need to be before being compressed.
    ///
    /// `None` to disable packet compression.
    pub compress_threshold : Option<u32>,

    /// Whether the Mojang authentication servers should be contacted to
    ///  confirm player's identities on join.
    ///
    /// **WARNING**
    /// Setting this to `false` allows any player to join with any username,
    ///  potentially letting them steal other player's data.
    pub mojauth_enabled    : bool

}

impl OctetConnPlugin {
    const DEFAULT_LISTEN_ADDRS : &[SocketAddr] = &[
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 25565)),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 25565))
    ];
}

impl Default for OctetConnPlugin {
    fn default() -> Self { Self {
        listen_addrs       : Cow::Borrowed(Self::DEFAULT_LISTEN_ADDRS),
        compress_threshold : Some(64),
        mojauth_enabled    : true
    } }
}

impl Plugin for OctetConnPlugin {
    fn build(&self, app : &mut App) {
        app
            .insert_resource(StartListenerConnPlugin(Some(self.clone())))
            .add_systems(Startup, start_listener);
    }
}


#[derive(Resource)]
struct StartListenerConnPlugin(Option<OctetConnPlugin>);

/// A connected peer.
#[expect(dead_code)] // TODO: the task value is currently unused. this will change.
#[derive(Component)]
pub struct ConnPeer(Task<error::ConnPeerResult>);


fn start_listener(
    mut config : ResMut<StartListenerConnPlugin>,
    mut cmds   : Commands
) {
    // Scuffed Resource-take because Bevy provides no way to use [`bevy_ecs::system::In`] here, nor a "`ResTake`" type.
    let config = config.0.take().unwrap();
    cmds.remove_resource::<StartListenerConnPlugin>();
    cmds.spawn_task(async move || {
        let Err(err) = run_listener(config).await;
        panic!("{err:?}"); // TODO: Handle error
    });
}

async fn run_listener(config : OctetConnPlugin) -> io::Result<Never> {
    let listener = TcpListener::bind(config.listen_addrs.as_ref()).await?;
    loop {
        let (stream, addr,) = listener.accept().await?;
        AsyncWorld.spawn_bundle((ConnPeer(AsyncWorld.spawn_task(async move {
            if let Err(err) = run_peer(
                comms::ConnPeerComms::new(stream, addr),
                config.compress_threshold,
                config.mojauth_enabled
            ).await {
                println!("{err:?}"); // TODO: Proper error handler;
            }
            Ok(())
        })),));
    }
}


async fn run_peer(
    mut comms              : comms::ConnPeerComms,
        compress_threshold : Option<u32>,
        mojauth_enabled    : bool
) -> error::ConnPeerResult {
    match (handshake::wait_for_intention(&mut comms).await?) {
        handshake::Intention::Status => status::handle_requests(&mut comms).await,
        handshake::Intention::Login  => {
            let player_id = login::handle_login_process(
                &mut comms,
                compress_threshold,
                mojauth_enabled
            ).await?;
            config_play::handle_config_play(player_id, &mut comms).await
        }
    }
}

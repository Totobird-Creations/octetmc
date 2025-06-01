//! Connection listener and client manager.


use crate::util::Never;
use octetmc_protocol::value::text::Text;
use core::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };
use std::borrow::Cow;
use std::io;
use smol::net::TcpListener;
use bevy_app::{ App, Plugin, Startup };
use bevy_ecs::component::Component;
use bevy_ecs::system::{ Commands, ResMut };
use bevy_ecs::resource::Resource;
use bevy_defer::{ AsyncWorld, Task, AsyncCommandsExtension };


mod comms;
use comms::ConnPeerComms;

pub(crate) mod event;


mod handshake;
use handshake::Intention;

mod status;

mod login;

mod config_play;


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
pub struct ConnPeer(Task<ConnPeerResult>);


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
                ConnPeerComms::new(stream, addr),
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
    mut comms              : ConnPeerComms,
        compress_threshold : Option<u32>,
        mojauth_enabled    : bool
) -> ConnPeerResult {
    match (handshake::wait_for_intention(&mut comms).await?) {
        Intention::Status => status::handle_requests(&mut comms).await,
        Intention::Login  => {
            login::handle_login_process(
                &mut comms,
                compress_threshold,
                mojauth_enabled
            ).await?;
            config_play::handle_config_play(&mut comms).await
        }
    }
}


/// The state of a client's connection.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum ConnPeerState {
    Handshake,
    Status,
    Login,
    ConfigPlay(ConfigPlay)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum ConfigPlay {
    Config { active_ticks : u8 },
    Play
}


/// A [`Result`] with a [`ConnPeerError`] error.
pub type ConnPeerResult<T = ()> = Result<T, ConnPeerError>;

/// An error raised by a connected peer.
#[derive(Debug)]
pub enum ConnPeerError {

    /// Declaring intention or logging in took too long, or a keepalive was not responded to in time.
    TimedOut,

    /// The client's protocol version does not match the server's.
    ProtocolMismatch {
        /// The protocol version that was sent by the client.
        client : u32,
        /// The expected protocol version.
        server : u32
    },

    /// The client sent too much data in too short of a time span.
    ReadQueueOverflow,

    /// An invalid packet length was received.
    InvalidPacketLength,

    /// A received packet is too long.
    PacketTooLong,

    /// A received packet has an unknown or unexpected ID.
    UnknownPacketPrefix(u8),

    /// A received packet could not be decoded.
    BadPacket(Cow<'static, str>),

    /// A received packet was longer than the decoder expected.
    NoPacketEnd,

    /// A received packet could not be decompressed.
    BadPacketZlib,

    /// The username that the client sent is too long.
    UsernameTooLong,

    /// Failed to exchange pkey and establish shared secret cipher.
    KeyExchangeFailed,

    /// The mojauth servers are currently unreachable.
    AuthServerUnreachable,

    /// The mojauth servers returned unrecognised data.
    BadAuthServer,

    /// The client's profile could not be verified.
    AuthFailed,

    /// The player is already logged in.
    AlreadyLoggedIn,

    /// The client closed the connection.
    PeerClosed,

    /// The server kicked the client.
    Kicked(Text<'static>),

    /// Some other IO error occured.
    Io(io::Error)

}
impl From<io::Error> for ConnPeerError {
    fn from(value : io::Error) -> Self { Self::Io(value) }
}

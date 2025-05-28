//! Connection listener and client manager.


use core::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };
use std::borrow::Cow;
use bevy_app::{ App, Plugin };


/// Enables the connection listener and client manager on install.
pub struct OctetConnPlugin {

    /// Addresses to listen on.
    ///
    /// The default port the game uses is `25565`.
    pub listen_addrs       : Cow<'static, [SocketAddr]>,

    /// How large packets need to be before being compressed.
    ///
    /// `None` to disable packet compression.
    pub compress_threshold : Option<usize>,

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
    fn build(&self, _app : &mut App) {
        todo!()
    }
}

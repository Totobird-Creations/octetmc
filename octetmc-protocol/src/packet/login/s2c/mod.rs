//! Packets sent by the server to the client in the login state.


use crate::packet::StateLogin;
use crate::packet::encode::packet_encode_group;


// TODO: login_disconnect

pub mod hello;

pub mod login_finished;

pub mod login_compression;

// TODO: custom_query

// TODO: cookie_request


packet_encode_group!{
    type State = StateLogin;
    /// `S2CLogin`-type packets.
    pub enum S2CLoginPackets<'l> {
        /// `HelloS2CLoginPacket`
        Hello(hello::HelloS2CLoginPacket<'l>),
        /// `LoginSuccessS2CLoginPacket`
        LoginSuccess(login_finished::LoginFinishedS2CLoginPacket<'l>),
        /// `LoginCompressionS2CLoginPacket`
        LoginCompression(login_compression::LoginCompressionS2CLoginPacket)
    }
}


impl S2CLoginPackets<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `S2CLoginPackets<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> S2CLoginPackets<'static> { match (self) {
        Self::Hello            (v) => S2CLoginPackets::Hello            (v.into_static_owned()),
        Self::LoginSuccess     (v) => S2CLoginPackets::LoginSuccess     (v.into_static_owned()),
        Self::LoginCompression (v) => S2CLoginPackets::LoginCompression (v)
    } }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `S2CLoginPackets<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> S2CLoginPackets<'static> { match (self) {
        Self::Hello            (v) => S2CLoginPackets::Hello            (v.to_static_owned()),
        Self::LoginSuccess     (v) => S2CLoginPackets::LoginSuccess     (v.to_static_owned()),
        Self::LoginCompression (v) => S2CLoginPackets::LoginCompression (*v)
    } }

}

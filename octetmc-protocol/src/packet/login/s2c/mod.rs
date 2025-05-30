//! Packets sent by the server to the client in the login state.


use crate::packet::StateLogin;
use crate::packet::encode::packet_encode_group;


pub mod hello;

pub mod login_compression;


packet_encode_group!{
    type State = StateLogin;
    /// `S2CLogin`-type packets.
    pub enum S2CLoginPackets<'l> {
        /// `HelloS2CLoginPacket`
        Hello(hello::HelloS2CLoginPacket<'l>),
        /// `LoginCompressionS2CLoginPacket`
        LoginCompression(login_compression::LoginCompressionS2CLoginPacket)
    }
}

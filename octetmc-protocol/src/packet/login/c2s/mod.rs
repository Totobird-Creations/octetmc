//! Packets sent by the client to the server in the login state.


use crate::packet::StateLogin;
use crate::packet::decode::packet_decode_group;
use crate::packet::decode::str::StringDecodeError;


pub mod hello;

pub mod key;


packet_decode_group!{
    type State     = StateLogin;
    type Error<'l> = StringDecodeError;
    /// `C2SLogin`-type packets.
    pub enum C2SLoginPackets<'l> {
        /// `HelloC2SLoginPacket`
        Hello(hello::HelloC2SLoginPacket<'l>),
        /// `KeyC2SLoginPacket`
        Key(key::KeyC2SLoginPacket<'l>)
    }
}

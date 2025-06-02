//! Packets sent by the client to the server in the login state.


use crate::packet::StateLogin;
use crate::packet::decode::packet_decode_group;
use crate::packet::decode::str::StringDecodeError;


pub mod hello;

pub mod key;

// TODO: custom_query_answer

pub mod login_acknowledged;

// TODO: cookie_response


packet_decode_group!{
    type State     = StateLogin;
    type Error<'l> = StringDecodeError;
    /// `C2SLogin`-type packets.
    pub enum C2SLoginPackets<'l> {
        /// `HelloC2SLoginPacket`
        Hello(hello::HelloC2SLoginPacket<'l>),
        /// `KeyC2SLoginPacket`
        Key(key::KeyC2SLoginPacket<'l>),
        /// `LoginAcknowledgedC2SLoginPacket`
        LoginAcknowledged(login_acknowledged::LoginAcknowledgedC2SLoginPacket)
    }
}


impl C2SLoginPackets<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `C2SLoginPackets<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> C2SLoginPackets<'static> { match (self) {
        Self::Hello             (v) => C2SLoginPackets::Hello             (v.into_static_owned()),
        Self::Key               (v) => C2SLoginPackets::Key               (v.into_static_owned()),
        Self::LoginAcknowledged (v) => C2SLoginPackets::LoginAcknowledged (v)
    } }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `C2SLoginPackets<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> C2SLoginPackets<'static> { match (self) {
        Self::Hello             (v) => C2SLoginPackets::Hello             (v.to_static_owned()),
        Self::Key               (v) => C2SLoginPackets::Key               (v.to_static_owned()),
        Self::LoginAcknowledged (v) => C2SLoginPackets::LoginAcknowledged (*v)
    } }

}

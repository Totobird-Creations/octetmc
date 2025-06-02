//! Packets sent by the client to the server in the handshake state.


use crate::packet::StateHandshake;
use crate::packet::decode::packet_decode_group;


pub mod intention;


packet_decode_group!{
    type State     = StateHandshake;
    type Error<'l> = intention::IntentionDecodeError;
    /// `C2SHandshake`-type packets.
    pub enum C2SHandshakePackets<'l> {
        /// `IntentionC2SHandshakePacket`
        Intention(intention::IntentionC2SHandshakePacket<'l>)
    }
}


impl C2SHandshakePackets<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `C2SHandshakePackets<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> C2SHandshakePackets<'static> { match (self) {
        Self::Intention (v) => C2SHandshakePackets::Intention (v.into_static_owned())
    } }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `C2SHandshakePackets<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> C2SHandshakePackets<'static> { match (self) {
        Self::Intention (v) => C2SHandshakePackets::Intention (v.to_static_owned())
    } }

}

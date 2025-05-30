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

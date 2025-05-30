use crate::packet::StateHandshake;
use crate::packet::decode::packet_decode_group;


pub mod intention;


packet_decode_group!{
    type State     = StateHandshake;
    type Error<'l> = intention::IntentionDecodeError;
    pub enum C2SHandshakePackets<'l> {
        Intention(intention::IntentionC2SHandshakePacket<'l>)
    }
}

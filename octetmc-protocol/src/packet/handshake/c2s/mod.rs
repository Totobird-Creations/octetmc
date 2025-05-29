use crate::packet::{ BoundC2S, StateHandshake, packet_group };


pub mod intention;


packet_group!{
    type Bound     = BoundC2S;
    type State     = StateHandshake;
    type Error<'l> = intention::IntentionDecodeError;
    pub enum C2SHandshakePackets<'l> {
        Intention(intention::IntentionC2SHandshakePacket<'l>)
    }
}

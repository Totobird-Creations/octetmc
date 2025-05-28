pub mod intention;


pub enum C2SHandshakePackets<'l> {
    Intention(intention::IntentionC2SHandshakePacket<'l>)
}

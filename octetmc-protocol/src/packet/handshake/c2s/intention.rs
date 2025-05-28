use std::borrow::Cow;


pub struct IntentionC2SHandshakePacket<'l> {
    pub protocol  : usize,
    pub address   : Cow<'l, str>,
    pub port      : u16,
    pub intention : Intention
}


pub enum Intention {
    Status   = 1,
    Login    = 2,
    Transfer = 3,
}

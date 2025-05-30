use crate::packet::StateLogin;
use crate::packet::encode::packet_encode_group;


packet_encode_group!{
    type State = StateLogin;
    pub enum S2CLoginPackets {
    }
}

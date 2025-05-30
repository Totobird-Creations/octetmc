use crate::packet::StateConfig;
use crate::packet::encode::packet_encode_group;


packet_encode_group!{
    type State = StateConfig;
    pub enum S2CConfigPackets {
    }
}

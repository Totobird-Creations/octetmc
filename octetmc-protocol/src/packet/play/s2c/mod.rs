use crate::packet::StatePlay;
use crate::packet::encode::packet_encode_group;


packet_encode_group!{
    type State = StatePlay;
    pub enum S2CPlayPackets {
    }
}

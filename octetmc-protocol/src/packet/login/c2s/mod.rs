use crate::packet::StateLogin;
use crate::packet::decode::{ IncompleteData, packet_decode_group };


packet_decode_group!{
    type State     = StateLogin;
    type Error<'l> = IncompleteData;
    pub enum C2SLoginPackets {
    }
}

use crate::packet::StatePlay;
use crate::packet::decode::{ IncompleteData, packet_decode_group };


packet_decode_group!{
    type State     = StatePlay;
    type Error<'l> = IncompleteData;
    pub enum C2SPlayPackets {
    }
}

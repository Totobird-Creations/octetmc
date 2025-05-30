use crate::packet::StateConfig;
use crate::packet::decode::{ IncompleteData, packet_decode_group };


packet_decode_group!{
    type State     = StateConfig;
    type Error<'l> = IncompleteData;
    pub enum C2SConfigPackets {
    }
}

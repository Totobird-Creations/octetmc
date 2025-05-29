use crate::packet::{ BoundC2S, StatePlay, packet_group };
use crate::packet::decode::IncompleteData;


packet_group!{
    type Bound     = BoundC2S;
    type State     = StatePlay;
    type Error<'l> = IncompleteData;
    pub enum C2SPlayPackets {
    }
}

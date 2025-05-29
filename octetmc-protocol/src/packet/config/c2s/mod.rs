use crate::packet::{ BoundC2S, StateConfig, packet_group };
use crate::packet::decode::IncompleteData;


packet_group!{
    type Bound     = BoundC2S;
    type State     = StateConfig;
    type Error<'l> = IncompleteData;
    pub enum C2SConfigPackets {
    }
}

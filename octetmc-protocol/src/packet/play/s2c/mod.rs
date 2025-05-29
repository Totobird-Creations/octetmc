use crate::packet::{ BoundS2C, StatePlay, packet_group };
use crate::packet::decode::IncompleteData;


packet_group!{
    type Bound     = BoundS2C;
    type State     = StatePlay;
    type Error<'l> = IncompleteData;
    pub enum S2CPlayPackets {
    }
}

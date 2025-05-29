use crate::packet::{ BoundS2C, StateStatus, packet_group };
use crate::packet::decode::IncompleteData;


packet_group!{
    type Bound     = BoundS2C;
    type State     = StateStatus;
    type Error<'l> = IncompleteData;
    pub enum S2CStatusPackets {
    }
}

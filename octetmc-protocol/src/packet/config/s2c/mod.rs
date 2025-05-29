use crate::packet::{ BoundS2C, StateConfig, packet_group };
use crate::packet::decode::IncompleteData;


packet_group!{
    type Bound     = BoundS2C;
    type State     = StateConfig;
    type Error<'l> = IncompleteData;
    pub enum S2CConfigPackets {
    }
}

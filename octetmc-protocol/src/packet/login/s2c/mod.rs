use crate::packet::{ BoundS2C, StateLogin, packet_group };
use crate::packet::decode::IncompleteData;


packet_group!{
    type Bound     = BoundS2C;
    type State     = StateLogin;
    type Error<'l> = IncompleteData;
    pub enum S2CLoginPackets {
    }
}

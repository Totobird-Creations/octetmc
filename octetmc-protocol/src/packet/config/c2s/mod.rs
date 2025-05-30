//! Packets sent by the client to the server in the config state.


use crate::packet::StateConfig;
use crate::packet::decode::{ IncompleteData, packet_decode_group };


packet_decode_group!{
    type State     = StateConfig;
    type Error<'l> = IncompleteData;
    /// `C2SConfig`-type packets.
    pub enum C2SConfigPackets {
    }
}

//! Packets sent by the client to the server in the play state.


use crate::packet::StatePlay;
use crate::packet::decode::{ IncompleteData, packet_decode_group };


packet_decode_group!{
    type State     = StatePlay;
    type Error<'l> = IncompleteData;
    /// `C2SPlay`-type packets.
    pub enum C2SPlayPackets {
    }
}

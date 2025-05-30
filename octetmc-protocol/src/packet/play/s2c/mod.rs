//! Packets sent by the server to the client in the play state.


use crate::packet::StatePlay;
use crate::packet::encode::packet_encode_group;


packet_encode_group!{
    type State = StatePlay;
    /// `S2CPlay`-type packets.
    pub enum S2CPlayPackets {
    }
}

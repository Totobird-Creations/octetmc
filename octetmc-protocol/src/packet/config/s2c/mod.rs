//! Packets sent by the server to the client in the config state.


use crate::packet::StateConfig;
use crate::packet::encode::packet_encode_group;


packet_encode_group!{
    type State = StateConfig;
    /// `S2CConfig`-type packets.
    pub enum S2CConfigPackets {
    }
}

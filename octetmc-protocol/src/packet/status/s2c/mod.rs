//! Packets sent by the server to the client in the status state.


use crate::packet::StateStatus;
use crate::packet::encode::packet_encode_group;


pub mod status_response;

pub mod pong_response;


packet_encode_group!{
    type State = StateStatus;
    /// `S2CStatus`-type packets.
    pub enum S2CStatusPackets<'l, 'k> {
        /// `StatusResponseS2CStatusPacket`
        StatusResponse(status_response::StatusResponseS2CStatusPacket<'l, 'k>),
        /// `PongResponseS2CStatusPacket`
        PingResponse(pong_response::PongResponseS2CStatusPacket)
    }
}

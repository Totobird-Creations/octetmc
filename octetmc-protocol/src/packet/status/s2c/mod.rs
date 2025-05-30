use crate::packet::StateStatus;
use crate::packet::encode::packet_encode_group;


pub mod status_response;


packet_encode_group!{
    type State = StateStatus;
    pub enum S2CLoginPackets<'a, 'b, 'c, 'd, 'e> {
        StatusResponse(status_response::StatusResponseS2CStatusPacket<'a, 'b, 'c, 'd, 'e>)
    }
}

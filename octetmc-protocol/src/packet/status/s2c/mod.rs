use crate::packet::StateStatus;
use crate::packet::encode::packet_encode_group;


mod status_response;


packet_encode_group!{
    type State = StateStatus;
    pub enum S2CLoginPackets<'l> {
        StatusResponse(status_response::StatusResponseS2CStatusPacket<'l>)
    }
}

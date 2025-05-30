use crate::packet::StateStatus;
use crate::packet::decode::{ IncompleteData, packet_decode_group };
use std::borrow::Cow;


pub mod status_request;

pub mod ping_request;


packet_decode_group!{
    type State     = StateStatus;
    type Error<'l> = C2SStatusPacketParseError;
    pub enum C2SStatusPackets {
        StatusRequest(status_request::StatusRequestC2SStatusPacket),
        PingRequest(ping_request::PingRequestC2SStatusPacket)
    }
}


pub enum C2SStatusPacketParseError {
    IncompleteData
}

impl From<C2SStatusPacketParseError> for Cow<'static, str> {
    fn from(value : C2SStatusPacketParseError) -> Self { match (value) {
        C2SStatusPacketParseError::IncompleteData => IncompleteData.into()
    } }
}

impl From<IncompleteData> for C2SStatusPacketParseError {
    fn from(_ : IncompleteData) -> Self { Self::IncompleteData }
}

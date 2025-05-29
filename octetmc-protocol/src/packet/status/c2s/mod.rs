pub mod status_request;

pub mod ping_request;


pub enum C2SStatusPackets {
    StatusRequest(status_request::StatusRequestC2SStatusPacket),
    PingRequest(ping_request::PingRequestC2SStatusPacket)
}

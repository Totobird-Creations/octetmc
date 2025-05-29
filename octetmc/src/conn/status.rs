use super::{ ConnPeerComms, ConnPeerResult, ConnPeerError };
use super::comms::{ ConnPeerState };
use octetmc_protocol::packet::status::c2s::C2SStatusPackets;
use octetmc_protocol::packet::status::c2s::ping_request::PingRequestC2SStatusPacket;
use core::time::Duration;


const REQUEST_TIMEOUT : Duration = Duration::from_millis(250);


pub(super) async fn handle_requests(comms : &mut ConnPeerComms) -> ConnPeerResult<()> {
    comms.set_state(ConnPeerState::Status);

    match (*comms.read_packet_timeout::<C2SStatusPackets>(REQUEST_TIMEOUT).await?) {

        C2SStatusPackets::StatusRequest(_) => {
            todo!();
        },

        C2SStatusPackets::PingRequest(PingRequestC2SStatusPacket { timestamp }) => {
            Err(ConnPeerError::OperationCompleted)
        }

    }
}

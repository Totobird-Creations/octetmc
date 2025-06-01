use super::error::{ ConnPeerResult, ConnPeerError };
use super::comms::ConnPeerComms;
use octetmc_protocol::PROTOCOL_VERSION;
use octetmc_protocol::packet::handshake::c2s::intention::{ IntentionC2SHandshakePacket, Intention as PtcIntention };
use core::time::Duration;


const INTENTION_TIMEOUT : Duration = Duration::from_secs(1);


pub(super) async fn wait_for_intention(comms : &mut ConnPeerComms) -> ConnPeerResult<Intention> {

    let packet = comms.read_packet_timeout::<IntentionC2SHandshakePacket>(INTENTION_TIMEOUT).await?;

    Ok(match (packet.intention) {
        PtcIntention::Status                         => Intention::Status,
        PtcIntention::Login | PtcIntention::Transfer => {
            if (packet.protocol != PROTOCOL_VERSION) {
                return Err(ConnPeerError::ProtocolMismatch { client : packet.protocol, server : PROTOCOL_VERSION });
            }
            Intention::Login
        }
    })

}


pub(super) enum Intention {
    Status,
    Login
}

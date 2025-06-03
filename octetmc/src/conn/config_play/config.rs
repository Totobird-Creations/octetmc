use super::{ ConfigPlay, ConnPeerComms, ConnPeerResult };
use super::in_message::ConnPeerInMessage;
use super::action::ConnPeerAction;
use crate::player::PlayerId;
use octetmc_protocol::packet::config::c2s::C2SConfigPackets;
use octetmc_protocol::packet::config::c2s::client_information::ClientInformationC2SConfigPacket;
use octetmc_protocol::packet::config::c2s::custom_payload::CustomPayloadC2SConfigPacket;
use octetmc_protocol::packet::config::s2c::finish_configuration::FinishConfigurationS2CConfigPacket;


pub(in super::super) async unsafe fn switch_to_play(player_id : PlayerId, comms : &mut ConnPeerComms) -> ConnPeerResult {
    match (unsafe { comms.state_assume_config_play() }) {

        ConfigPlay::Config { .. } => {
            comms.send_packet(&FinishConfigurationS2CConfigPacket).await?;
            // TODO: Put a timeout on the flush.
            while let ConfigPlay::Config { .. } = unsafe { comms.state_assume_config_play() } {
                handle_config_packet(player_id, comms.read_packet::<C2SConfigPackets>().await?).await?.handle(comms).await?;
            }
            Ok(())
        },

        ConfigPlay::Play => Ok(())

    }
}


pub(super) async fn handle_config_packet(_player_id : PlayerId, packet : C2SConfigPackets<'_>) -> ConnPeerResult<ConnPeerAction> {
    match (packet) {

        C2SConfigPackets::ClientInformation(ClientInformationC2SConfigPacket { info }) => {
            Ok(ConnPeerAction::SendInMessage(ConnPeerInMessage::SetClientInfo(info.into_static_owned())))
        }

        C2SConfigPackets::CustomPayload(CustomPayloadC2SConfigPacket { data }) => {
            Ok(ConnPeerAction::SendInMessage(ConnPeerInMessage::RecieveChannelData(data.into_static_owned())))
        }

        C2SConfigPackets::FinishConfiguration(_) => Ok(ConnPeerAction::SetState(ConfigPlay::Play))

    }
}

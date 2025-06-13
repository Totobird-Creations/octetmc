use super::error::ConnPeerResult;
use super::state::ConfigPlay;
use super::comms::ConnPeerComms;
use crate::player::PlayerId;
use octetmc_protocol::packet::config::c2s::C2SConfigPackets;
use octetmc_protocol::packet::play::c2s::C2SPlayPackets;
use bevy_defer::AsyncWorld;


pub(super) mod config;
pub use config::ConnInConfig;

pub(super) mod play;
pub use play::ConnInPlay;

pub(crate) mod out_message;
pub(crate) mod in_message;
pub(crate) mod action;


pub(super) async fn handle_config_play(player_id : PlayerId, comms : &mut ConnPeerComms) -> ConnPeerResult {
    loop {


        match (*unsafe { comms.state_assume_config_play() }) {
            ConfigPlay::Config { .. } => {

                if let Some(packet) = comms.try_read_packet::<C2SConfigPackets>()? {
                    config::handle_config_packet(player_id, packet).await?.handle(comms).await?;
                }

            },
            ConfigPlay::Play => {

                if let Some(packet) = comms.try_read_packet::<C2SPlayPackets>()? {
                    play::handle_play_packet(player_id, packet).await?.handle(comms).await?;
                }

            },
        }


        while let Some(event) = comms.try_read_out_message()? {
            event.handle(player_id, comms).await?;
        }


        AsyncWorld.yield_now().await;
    }
}

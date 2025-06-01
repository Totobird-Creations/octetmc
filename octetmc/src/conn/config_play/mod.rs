use super::error::ConnPeerResult;
use super::state::ConfigPlay;
use super::comms::ConnPeerComms;
use octetmc_protocol::packet::config::c2s::C2SConfigPackets;
use octetmc_protocol::packet::play::c2s::C2SPlayPackets;
use bevy_defer::AsyncWorld;


pub(super) mod config;

pub(super) mod play;

pub(crate) mod event;
pub(crate) mod action;


pub(super) async fn handle_config_play(comms : &mut ConnPeerComms) -> ConnPeerResult {
    loop {


        match (*unsafe { comms.state_assume_config_play() }) {
            ConfigPlay::Config { .. } => {

                if let Some(packet) = comms.try_read_packet::<C2SConfigPackets>()? {
                    config::handle_config_packet(packet).await?.handle(comms).await?;
                }

            },
            ConfigPlay::Play => {

                if let Some(packet) = comms.try_read_packet::<C2SPlayPackets>()? {
                    play::handle_play_packet(packet).await?.handle(comms).await?;
                }

            },
        }


        while let Some(event) = comms.try_read_event()? {
            event.handle(comms).await?;
        }


        AsyncWorld.yield_now().await;
    }
}

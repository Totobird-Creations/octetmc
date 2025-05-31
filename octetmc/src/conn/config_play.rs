use super::{ ConfigPlay, ConnPeerComms, ConnPeerResult };
use octetmc_protocol::packet::config::c2s::C2SConfigPackets;
use octetmc_protocol::packet::play::c2s::C2SPlayPackets;
use bevy_defer::AsyncWorld;


pub(super) async fn handle_config_play(comms : &mut ConnPeerComms) -> ConnPeerResult {
    loop {


        match (*unsafe { comms.state_assume_config_play() }) {
            ConfigPlay::Config { .. } => {

                if let Some(packet) = comms.try_read_packet::<C2SConfigPackets>()? {
                    println!("{:?}", packet.get());
                }

            },
            ConfigPlay::Play => {

                if let Some(packet) = comms.try_read_packet::<C2SPlayPackets>()? {
                    println!("{:?}", packet.get());
                }

            },
        }


        if let Some(event) = comms.try_read_event()? {
            event.handle(comms).await?;
        }


        AsyncWorld.yield_now().await;
    }
}

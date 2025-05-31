use super::{ ConnPeerState, ConfigPlay, ConnPeerComms, ConnPeerResult, ConnPeerError };
use octetmc_protocol::packet::config::c2s::C2SConfigPackets;
use octetmc_protocol::packet::play::c2s::C2SPlayPackets;


pub(super) async fn handle_config_play(comms : &mut ConnPeerComms) -> ConnPeerResult {
    todo!();

    // match (unsafe { comms.state_assume_config_play() }) {
    //     ConfigPlay::Config { active_ticks } => {
    //     },
    //     ConfigPlay::Play => todo!(),
    // }
}

use super::{ ConfigPlay, ConnPeerComms, ConnPeerResult };
use super::action::ConnPeerAction;
use crate::player::PlayerId;
use octetmc_protocol::packet::play::c2s::C2SPlayPackets;
use bevy_ecs::component::Component;


/// A Bevy ECS marker component indicating that a player is currently in the play state.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ConnInPlay;


pub(in super::super) async unsafe fn switch_to_config(_player_id : PlayerId, comms : &mut ConnPeerComms) -> ConnPeerResult {
    match (unsafe { comms.state_assume_config_play() }) {

        ConfigPlay::Config { active_ticks } => {
            *active_ticks = 0;
            Ok(())
        },

        ConfigPlay::Play => {
            // comms.send_packet(&StartConfigurationS2CPlayPacket).await?;
            // // TODO: Put a timeout on the flush.
            // while let ConfigPlay::Play { .. } = unsafe { comms.state_assume_config_play() } {
            //     handle_config_packet(comms.read_packet::<C2SPlayPackets>().await?).await?.handle(comms).await?;
            // }
            // let entity = AsyncWorld.entity(*player_id);
            // _ = entity.remove::<ConnInPlay>();
            // _ = entity.insert(ConnInConfig);
            todo!();
        }

    }
}


pub(super) async fn handle_play_packet(_player_id : PlayerId, packet : C2SPlayPackets) -> ConnPeerResult<ConnPeerAction> {
    match (packet) {

    }
}

use super::{ ConfigPlay, ConnPeerComms, ConnPeerResult, config };
use crate::player::PlayerId;


pub(crate) enum ConnPeerOutMessage {
    Tick

}


impl ConnPeerOutMessage {
    pub(super) async fn handle(self, player_id : PlayerId, comms : &mut ConnPeerComms) -> ConnPeerResult { match (self) {

        Self::Tick => {
            if let ConfigPlay::Config { active_ticks } = unsafe { comms.state_assume_config_play() } {
                // 2 ticks is the maximum duration to read a Bevy event.
                // All config events should be handled by now.
                if (*active_ticks >= 1) {
                    unsafe { config::switch_to_play(player_id, comms) }.await?;
                } else { *active_ticks += 1; }
            }
            Ok(())
        },

    } }
}

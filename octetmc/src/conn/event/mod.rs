use super::{ ConfigPlay, ConnPeerComms, ConnPeerResult };


pub(crate) enum ConnPeerEvent {
    Tick

}


impl ConnPeerEvent {
    pub(super) async fn handle(self, comms : &mut ConnPeerComms) -> ConnPeerResult { match (self) {

        Self::Tick => {
            if let ConfigPlay::Config { active_ticks } = unsafe { comms.state_assume_config_play() } {
                // 2 ticks is the maximum duration to read a Bevy event.
                // All config events should be handled by now.
                if (*active_ticks >= 1) {
                    unsafe { comms.switch_state_play() }.await?;
                } else { *active_ticks += 1; }
            }
            Ok(())
        },

    } }
}

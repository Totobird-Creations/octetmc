use super::{ ConfigPlay, ConnPeerComms, ConnPeerResult };


pub(crate) enum ConnPeerEvent {
    Tick

}


impl ConnPeerEvent {
    pub(super) async fn handle(self, comms : &mut ConnPeerComms) -> ConnPeerResult { match (self) {

        Self::Tick => {
            if let ConfigPlay::Config { active_ticks } = unsafe { comms.state_assume_config_play() } {
                if (*active_ticks >= 1) {
                    todo!("switch to play");
                } else { *active_ticks += 1; }
            }
            Ok(())
        },

    } }
}

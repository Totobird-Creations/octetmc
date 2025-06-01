use super::{ ConfigPlay, ConnPeerComms, ConnPeerResult };


pub(super) enum ConnPeerAction {

    SetState(ConfigPlay)

}


impl ConnPeerAction {
    pub(super) async fn handle(self, comms : &mut ConnPeerComms) -> ConnPeerResult { match (self) {

        Self::SetState(state) => {
            comms.set_state_config_play(state);
            Ok(())
        },

    } }
}

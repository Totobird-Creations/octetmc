use super::{ ConfigPlay, ConnPeerComms, ConnPeerResult };
use super::in_message::ConnPeerInMessage;


pub(super) enum ConnPeerAction {
    None,

    SetState(ConfigPlay),

    SendInMessage(ConnPeerInMessage)

}


impl ConnPeerAction {
    pub(super) async fn handle(self, comms : &mut ConnPeerComms) -> ConnPeerResult { match (self) {
        Self::None => Ok(()),

        Self::SetState(state) => {
            comms.set_state_config_play(state);
            Ok(())
        },

        Self::SendInMessage(message) => {
            comms.send_in_message(message).await; // TODO: Handle error
            Ok(())
        }

    } }
}

use super::{ ConnPeerState, ConnPeerComms, ConnPeerResult, ConnPeerError };
use core::time::Duration;


const LOGIN_TIMEOUT : Duration = Duration::from_millis(250);


pub(super) async fn handle_login_process(comms : &mut ConnPeerComms) -> ConnPeerResult {
    comms.set_state(ConnPeerState::Login);

    todo!()
}

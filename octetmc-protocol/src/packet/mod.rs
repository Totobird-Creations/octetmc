pub mod handshake;

pub mod status;

pub mod login;

pub mod config;

pub mod play;


pub mod decode;

pub mod encode;


/// ### Safety
/// You probably shouldn't be implementing this.
#[diagnostic::on_unimplemented(
    label = "`{Self}` is not a packet state",
    message = "`{Self}` is not a packet state"
)]
pub unsafe trait PacketState { }
pub struct StateHandshake;
unsafe impl PacketState for StateHandshake { }
pub struct StateStatus;
unsafe impl PacketState for StateStatus { }
pub struct StateLogin;
unsafe impl PacketState for StateLogin { }
pub struct StateConfig;
unsafe impl PacketState for StateConfig { }
pub struct StatePlay;
unsafe impl PacketState for StatePlay { }

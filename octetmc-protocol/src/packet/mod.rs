pub mod handshake;

pub mod status;

pub mod login;

pub mod config;

pub mod play;


pub mod decode;


#[diagnostic::on_unimplemented(
    label = "`{Self}` is not a packet bound",
    message = "`{Self}` is not a packet bound"
)]
pub unsafe trait PacketBound { }
pub struct BoundC2S;
unsafe impl PacketBound for BoundC2S { }
pub struct BoundS2C;
unsafe impl PacketBound for BoundS2C { }

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

#[diagnostic::on_unimplemented(
    label = "`{Self}` is not a valid packet bound-state pair",
    message = "`{Self}` is not a valid packet bound-state pair"
)]
pub unsafe trait PacketBoundState {}
unsafe impl PacketBoundState for (BoundC2S, StateHandshake,) { }
unsafe impl PacketBoundState for (BoundC2S, StateStatus,) { }
unsafe impl PacketBoundState for (BoundS2C, StateStatus,) { }
unsafe impl PacketBoundState for (BoundC2S, StateLogin,) { }
unsafe impl PacketBoundState for (BoundS2C, StateLogin,) { }
unsafe impl PacketBoundState for (BoundC2S, StateConfig,) { }
unsafe impl PacketBoundState for (BoundS2C, StateConfig,) { }
unsafe impl PacketBoundState for (BoundC2S, StatePlay,) { }
unsafe impl PacketBoundState for (BoundS2C, StatePlay,) { }


pub struct BufHead {
    head : usize
}

impl BufHead {

    #[inline(always)]
    pub fn new() -> Self { Self { head : 0 } }

    #[inline(always)]
    pub fn consumed(&self) -> usize { self.head }

}

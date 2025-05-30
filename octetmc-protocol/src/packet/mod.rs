//! Packet definitions and en?decoders.


pub mod handshake;

pub mod status;

pub mod login;

pub mod config;

pub mod play;


pub mod decode;

pub mod encode;


/// The state that a packet is sent in.
///
/// This is used by the en/decode traits to restrict where they can be used.
///
/// ### Safety
/// You probably shouldn't be implementing this.
#[diagnostic::on_unimplemented(
    label = "`{Self}` is not a packet state",
    message = "`{Self}` is not a packet state"
)]
pub unsafe trait PacketState { }

/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshaking
pub struct StateHandshake;
unsafe impl PacketState for StateHandshake { }

/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status
pub struct StateStatus;
unsafe impl PacketState for StateStatus { }

/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login
pub struct StateLogin;
unsafe impl PacketState for StateLogin { }

/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Configuration
pub struct StateConfig;
unsafe impl PacketState for StateConfig { }

/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Play
pub struct StatePlay;
unsafe impl PacketState for StatePlay { }

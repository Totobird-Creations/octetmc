//! Packet definitions and en?decoders.


use core::fmt;


pub mod handshake;

pub mod status;

pub mod login;

pub mod config;

pub mod play;

mod prefix_check;


pub mod decode;

pub mod encode;


/// The state that a packet is sent in.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PacketState {

    /// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshaking>
    Handshake,

    /// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status>
    Status,

    /// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login>
    Login,

    /// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Configuration>
    Config,

    /// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Play>
    Play

}


/// A packet.
#[expect(private_bounds)]
pub trait Packet : crate::Sealed { }

/// A packet group.
#[expect(private_bounds)]
pub trait PacketGroup : crate::Sealed { }


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
pub unsafe trait AsPacketState : Clone + Copy + fmt::Debug {
    /// Returns `Self` as a `PacketState` variant.
    fn as_packet_state() -> PacketState;
}

/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshaking>
#[derive(Clone, Copy, Debug)]
pub struct StateHandshake;
unsafe impl AsPacketState for StateHandshake {
    #[inline(always)]
    fn as_packet_state() -> PacketState { PacketState::Handshake }
}

/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status>
#[derive(Clone, Copy, Debug)]
pub struct StateStatus;
unsafe impl AsPacketState for StateStatus {
    #[inline(always)]
    fn as_packet_state() -> PacketState { PacketState::Status }
}

/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login>
#[derive(Clone, Copy, Debug)]
pub struct StateLogin;
unsafe impl AsPacketState for StateLogin {
    #[inline(always)]
    fn as_packet_state() -> PacketState { PacketState::Login }
}

/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Configuration>
#[derive(Clone, Copy, Debug)]
pub struct StateConfig;
unsafe impl AsPacketState for StateConfig {
    #[inline(always)]
    fn as_packet_state() -> PacketState { PacketState::Config }
}

/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Play>
#[derive(Clone, Copy, Debug)]
pub struct StatePlay;
unsafe impl AsPacketState for StatePlay {
    #[inline(always)]
    fn as_packet_state() -> PacketState { PacketState::Play }
}


/// `u8`-similar types.
pub trait Byte : Copy {
    /// Return `self` as a `u8`.
    fn as_be_byte(self) -> u8;
}

impl Byte for u8 {
    #[inline(always)]
    fn as_be_byte(self) -> u8 { self.to_be() }
}

impl Byte for i8 {
    #[inline(always)]
    fn as_be_byte(self) -> u8 { self.to_be().cast_unsigned() }
}

impl Byte for bool {
    #[inline(always)]
    fn as_be_byte(self) -> u8 { <u8 as Byte>::as_be_byte(if (self) { 1 } else { 0 }) }
}

impl<T> Byte for &T
where
    T : Byte
{
    #[inline(always)]
    fn as_be_byte(self) -> u8 { <T as Byte>::as_be_byte(*self) }
}

//! Player signal and update events.


use super::PlayerId;
use crate::util::macros::deref_single;
use octetmc_protocol::value::ident::Ident;
use bevy_ecs::event::Event;


deref_single!{
    /// A client's information, settings, or brand have been updated.
    #[derive(Event)]
    pub struct PlayerInfoUpdated(PlayerId);
}


/// A client sent a plugin message.
#[derive(Event)]
pub struct PlayerChannelDataReceived {

    /// The ID of the player who sent this message.
    pub player_id : PlayerId,

    /// The ID of the channel.
    pub channel   : Ident<'static>,

    /// The message data.
    pub data      : Vec<u8>

}

//! Player information and operations.


use crate::conn;
use crate::conn::out_message::ConnPeerOutMessage;
use crate::conn::in_message::ConnPeerInMessage;
use crate::world::dimension::Dimension;
use crate::util::CratePrivateNew;
use crate::util::macros::deref_single;
use octetmc_protocol::value::ident::Ident;
use octetmc_protocol::value::game_mode::GameMode;
use octetmc_protocol::value::profile::PlayerProfile;
use octetmc_protocol::value::client_info::ClientInfo;
use octetmc_protocol::packet::config::s2c::registry_data::RegistryEntry;
use core::ops::Deref;
use bevy_app::{ Plugin, App, Update };
use bevy_ecs::entity::Entity;
use bevy_ecs::component::Component;
use bevy_ecs::system::{ Query, Commands };
use bevy_ecs::resource::Resource;
use smol::channel;


pub mod login;

pub mod info;


/// A player connected to the server.
#[derive(Component)]
pub struct Player {

    conn_out_sender  : channel::Sender<ConnPeerOutMessage>,
    conn_in_receiver : channel::Receiver<ConnPeerInMessage>,

    profile          : PlayerProfile<'static>,
    info             : Option<ClientInfo<'static>>,
    brand            : Option<String>

}

impl Player {

    #[inline(always)]
    pub(crate) fn new(
        conn_out_sender  : channel::Sender<ConnPeerOutMessage>,
        conn_in_receiver : channel::Receiver<ConnPeerInMessage>,
        profile          : PlayerProfile<'static>
    ) -> Self { Self {
        conn_out_sender,
        conn_in_receiver,
        profile,
        info             : None,
        brand            : None
    } }

    #[inline]
    pub(crate) fn try_read_in_message(&self) -> Result<ConnPeerInMessage, channel::TryRecvError> {
        self.conn_in_receiver.try_recv()
    }

}

impl Player {

    /// Returns the player's profile.
    #[inline]
    pub fn profile(&self) -> &PlayerProfile { &self.profile }


    /// Returns the player's client info, if available.
    #[inline]
    pub fn info(&self) -> Option<&ClientInfo<'_>> { self.info.as_ref() }

    #[inline]
    pub(crate) fn set_info(&mut self, info : ClientInfo<'static>) { self.info = Some(info); }


    /// Returns the player's client brand, if available.
    #[inline]
    pub fn brand(&self) -> Option<&str> { self.brand.as_deref() }

    #[inline]
    pub(crate) fn set_brand(&mut self, brand : String) { self.brand = Some(brand); }

}

impl Player {

    /// Overwrites a player's registry.
    pub fn set_registry(&self, id : Ident<'static>, entries : Vec<RegistryEntry<'static>>) {
        self.conn_out_sender.force_send(ConnPeerOutMessage::SetRegistry { id, entries }); // TODO: Handle error
    }

    /// Logs the player in. This can be called **once** per player.
    ///
    /// ### Panics
    /// Panics if called a second time on the same player.
    pub fn login(&self,
        is_hardcore        : bool,
        dimension          : Dimension<'static>,
        reduced_debug_info : bool,
        respawn_screens    : bool,
        game_mode          : GameMode
    ) {
        self.conn_out_sender.force_send(ConnPeerOutMessage::Login {
            is_hardcore,
            dimension,
            reduced_debug_info,
            respawn_screens,
            game_mode
        }); // TODO: Handle error
    }

}


/// The current number of players connected to the server.
///
/// The current player count is displayed in the server list
///  if this resource exists.
#[derive(Resource)]
pub struct PlayerCount(pub(crate) u32);

impl Deref for PlayerCount {
    type Target = u32;
    fn deref(&self) -> &Self::Target { &self.0 }
}


deref_single!{
    /// The maximum number of players allowed to be connected
    ///  to the server at the same time.
    ///
    /// The max player count is displayed in the server list.
    #[derive(Resource)]
    pub struct MaxPlayerCount(u32);
}


deref_single!{
    /// An [`Entity`] wrapper, intended for [`Player`]s.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct PlayerId(Entity);
    From;
}


/// Bevy [`Plugin`] for tracking players.
pub struct OctetPlayerPlugin {

    /// Whether to track the player count for the server status.
    pub track_player_count : bool,

    /// The maximum number of players allowed to be connected
    ///  to the server at the same time.
    pub max_player_count   : Option<u32>

}

impl Default for OctetPlayerPlugin {
    fn default() -> Self { Self {
        track_player_count : true,
        max_player_count   : Some(100)
    } }
}

impl Plugin for OctetPlayerPlugin {
    fn build(&self, app : &mut App) {
        app .add_event::<login::PlayerLoggingInEvent>()
            .add_event::<login::KickPlayer>()
            .add_event::<info::PlayerInfoUpdated>()
            .add_event::<info::PlayerChannelDataReceived>()
            .add_systems(Update, tick_player_conns)
            .add_systems(Update, conn::in_message::handle_in_messages);
        if (self.track_player_count) {
            app.insert_resource(PlayerCount(0));
        }
        if let Some(max_player_count) = self.max_player_count {
            app.insert_resource(MaxPlayerCount::crate_private_new(max_player_count));
        }
    }
}

fn tick_player_conns(
    mut cmds      : Commands,
        q_players : Query<(Entity, &Player,)>
) {
    for (entity, player,) in &q_players {
        if (player.conn_out_sender.force_send(ConnPeerOutMessage::Tick).is_err()) {
            cmds.entity(entity).despawn();
        }
    }
}


/// Common imports.
pub mod prelude {
    pub use super::{
        Player,
        PlayerId,
        login::{
            PlayerLoggingInEvent,
            KickPlayer
        }
    };
}

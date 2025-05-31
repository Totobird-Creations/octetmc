//! Player login request events.


use super::{ Player, PlayerId };
use octetmc_protocol::value::text::{ Text, TextComponent, TextContent, TextStyle, TextInteract };
use std::borrow::Cow;
use bevy_app::{ App, Plugin, Update };
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Query;
use bevy_ecs::event::{ Event, EventReader, EventWriter };


/// A player logged in.
#[derive(Event)]
pub struct PlayerLoginEvent {

    /// The [`Entity`] ID of the [`Player`] who joined.
    pub player : PlayerId

}


/// Kick a player from the server.
#[derive(Event)]
pub struct KickPlayer {

    /// The [`Entity`] ID of the [`Player`] to kick.
    pub player : PlayerId,

    /// The message to display to the client.
    pub reason : Text<'static>

}


/// Bevy [`Plugin`] which kicks duplicate players.
#[derive(Default)]
pub struct PlayerKickDupesPlugin;

impl Plugin for PlayerKickDupesPlugin {
    fn build(&self, app : &mut App) {
        app.add_systems(Update, kick_dupe_players);
    }
}


const DEFAULT_DUPE_KICK_MESSAGE : Text<'static> = Text { components : Cow::Borrowed(&[
    TextComponent {
        content  : TextContent::Translate {
            key      : Cow::Borrowed("multiplayer.disconnect.duplicate_login"),
            fallback : None,
            with     : Cow::Borrowed(&[])
        },
        style    : TextStyle::NONE,
        interact : TextInteract::NONE,
        extra    : Cow::Borrowed(&[])
    }
]) };

fn kick_dupe_players(
    mut er_login  : EventReader<PlayerLoginEvent>,
    mut ew_kick   : EventWriter<KickPlayer>,
        q_players : Query<(Entity, &Player,)>
) {
    ew_kick.write_batch(er_login.read().filter_map(|event| {
        if let Ok((_, player,)) = (q_players.get(*event.player)) {
            Some(q_players.iter().filter_map(|(other_id, other_player,)| {
                if ((other_id != *event.player) && (other_player.profile.uuid == player.profile.uuid)) {
                    Some(KickPlayer { player : PlayerId::from(other_id), reason : DEFAULT_DUPE_KICK_MESSAGE })
                } else { None }
            }))
        } else { None }
    }).flatten());
}

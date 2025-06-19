//! Packets sent by the server to the client in the play state.


use crate::packet::StatePlay;
use crate::packet::encode::packet_encode_group;

// TODO: bundle_delimiter

// TODO: add_entity

// TODO: animate

// TODO: award_stats

// TODO: block_changed_ack

// TODO: block_destruction

// TODO: block_entity_data

// TODO: block_event

// TODO: block_update

// TODO: boss_event

// TODO: change_difficulty

// TODO: chunk_batch_finished

// TODO: chunk_batch_start

// TODO: chunks_biomes

// TODO: clear_titles

// TODO: command_suggestions

// TODO: commands

// TODO: container_close

// TODO: container_set_content

// TODO: container_set_data

// TODO: container_set_slot

// TODO: cookie_request

// TODO: cooldown

// TODO: custom_chat_completions

// TODO: custom_payload

// TODO: damage_event

// TODO: debug_sample

// TODO: delete_chat

// TODO: disconnect

// TODO: disguised_chat

// TODO: entity_event

// TODO: entity_position_sync

// TODO: explode

// TODO: forget_level_chunk

pub mod game_event;

// TODO: horse_screen_open

// TODO: hurt_animation

// TODO: initialise_border

// TODO: keep_alive

// TODO: level_chunk_with_light

// TODO: level_event

// TODO: level_particles

// TODO: light_update

pub mod login;

// TODO: map_item_data

// TODO: merchant_offers

// TODO: move_entity_pos

// TODO: move_entity_pos_rot

// TODO: move_minecart_along_track

// TODO: move_entity_rot

// TODO: move_vehicle

// TODO: open_book

// TODO: open_screen

// TODO: open_sign_editor

// TODO: ping

// TODO: pong_response

// TODO: place_ghost_recipe

// TODO: player_abilities

// TODO: player_chat

// TODO: player_combat_end

// TODO: player_combat_enter

// TODO: player_combat_kill

// TODO: player_info_remove

// TODO: player_info_update

// TODO: player_look_at

// TODO: player_position

// TODO: player_rotation

// TODO: recipe_book_add

// TODO: recipe_book_remove

// TODO: recipe_book_settings

// TODO: remove_entities

// TODO: remove_mob_effect

// TODO: reset_score

// TODO: resource_pack_pop

// TODO: resource_pack_push

// TODO: respawn

// TODO: rotate_head

// TODO: section_blocks_update

// TODO: select_advancements_tab

// TODO: server_data

// TODO: set_action_bar_text

// TODO: set_border_center

// TODO: set_border_lerp_size

// TODO: set_border_size

// TODO: set_border_warning_delay

// TODO: set_border_warning_distance

// TODO: set_camera

pub mod set_chunk_cache_centre;

pub mod set_chunk_cache_radius;

// TODO: set_cursor_item

// TODO: set_default_spawn_position

// TODO: set_display_objective

// TODO: set_entity_data

// TODO: set_entity_link

// TODO: set_entity_motion

// TODO: set_equipment

// TODO: set_experience

// TODO: set_health

// TODO: set_held_slot

// TODO: set_objective

// TODO: set_passengers

// TODO: set_player_inventory

// TODO: set_player_team

// TODO: set_score

// TODO: set_simulation_distance

// TODO: set_subtitle_text

// TODO: set_time

// TODO: set_title_text

// TODO: set_titles_animation

// TODO: sound_entity

// TODO: sound

// TODO: start_configuration

// TODO: stop_sound

// TODO: store_cookie

// TODO: system_chat

// TODO: tab_list

// TODO: tag_query

// TODO: take_item_entity

// TODO: teleport_entity

// TODO: test_instance_block_status

// TODO: ticking_state

// TODO: ticking_step

// TODO: transfer

// TODO: update_advancements

// TODO: update_attributes

// TODO: update_mob_effect

// TODO: update_recipes

// TODO: update_tags

// TODO: projectile_power

// TODO: custom_report_details

// TODO: server_links

// TODO: waypoint

// TODO: clear_dialog

// TODO: show_dialog


packet_encode_group!{
    type State = StatePlay;
    /// `S2CPlay`-type packets.
    pub enum S2CPlayPackets<'l> {
        /// `GameEventS2CPlayPacket`
        GameEvent(game_event::GameEventS2CPlayPacket),
        /// `LoginS2CPlayPacket`
        Login(login::LoginS2CPlayPacket<'l>),
        /// `SetChunkCacheCentre`
        SetChunkCacheCentre(set_chunk_cache_centre::SetChunkCacheCentreS2CPlayPacket),
        /// `SetChunkCacheRadius`
        SetChunkCacheRadius(set_chunk_cache_radius::SetChunkCacheRadiusS2CPlayPacket)
    }
}


impl S2CPlayPackets<'_> {

    /// Convert the inner parts of this packet to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `S2CPlayPackets<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> S2CPlayPackets<'static> { match (self) {
        Self::GameEvent           (v) => S2CPlayPackets::GameEvent(v),
        Self::Login               (v) => S2CPlayPackets::Login (v.into_static_owned()),
        Self::SetChunkCacheCentre (v) => S2CPlayPackets::SetChunkCacheCentre(v),
        Self::SetChunkCacheRadius (v) => S2CPlayPackets::SetChunkCacheRadius(v)
    } }

    /// Convert the inner parts of this packet to their owned counterparts.
    ///  Returns the newly created `S2CPlayPackets<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> S2CPlayPackets<'static> { match (self) {
        Self::GameEvent           (v) => S2CPlayPackets::GameEvent(*v),
        Self::Login               (v) => S2CPlayPackets::Login (v.to_static_owned()),
        Self::SetChunkCacheCentre (v) => S2CPlayPackets::SetChunkCacheCentre(*v),
        Self::SetChunkCacheRadius (v) => S2CPlayPackets::SetChunkCacheRadius(*v)
    } }

}

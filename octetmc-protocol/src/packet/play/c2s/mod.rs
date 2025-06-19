//! Packets sent by the client to the server in the play state.


use crate::packet::StatePlay;
use crate::packet::decode::{ IncompleteData, packet_decode_group };


// TODO: accept_teleportation

// TODO: block_entity_tag_query

// TODO: bundle_item_selected

// TODO: change_difficulty

// TODO: change_game_mode

// TODO: chat_ack

// TODO: chat_command

// TODO: chat_command_signed

// TODO: chat

// TODO: chat_session_update

// TODO: chunk_batch_received

// TODO: client_command

// TODO: client_tick_end

// TODO: client_information

// TODO: command_suggestion

// TODO: configuration_acknowledged

// TODO: container_button_click

// TODO: container_click

// TODO: container_close

// TODO: container_slot_state_changed

// TODO: cookie_response

// TODO: custom_payload

// TODO: debug_sample_subscription

// TODO: edit_book

// TODO: entity_tag_query

// TODO: interact

// TODO: jigsaw_generate

// TODO: keep_alive

// TODO: lock_difficulty

// TODO: move_player_pos

// TODO: move_player_pos_rot

// TODO: move_player_rot

// TODO: player_player_status_only

// TODO: move_vehicle

// TODO: paddle_boat

// TODO: pick_item_from_block

// TODO: pink_item_from_entity

// TODO: ping_request

// TODO: place_recipe

// TODO: player_abilities

// TODO: player_action

// TODO: player_command

// TODO: player_input

// TODO: player_loaded

// TODO: pong

// TODO: recipe_book_change_settings

// TODO: recipe_book_seen_recipe

// TODO: rename_item

// TODO: resource_pack

// TODO: seen_advancements

// TODO: select_trade

// TODO: set_carried_item

// TODO: set_command_block

// TODO: set_command_minecart

// TODO: set_creative_mode_slot

// TODO: set_jigsaw_block

// TODO: set_structure_block

// TODO: set_test_block

// TODO: sign_update

// TODO: swing

// TODO: teleport_to_entity

// TODO: test_instance_block_action

// TODO: use_item_on

// TODO: use_item

// TODO: custom_click_action


packet_decode_group!{
    type State     = StatePlay;
    type Error<'l> = IncompleteData;
    /// `C2SPlay`-type packets.
    pub enum C2SPlayPackets {
    }
}

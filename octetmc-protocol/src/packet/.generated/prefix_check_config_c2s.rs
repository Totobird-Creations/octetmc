macro_rules! prefix_check_config_c2s {
( custom_click_action, 0x08 $(,)? ) => { 8 };
( custom_payload, 0x02 $(,)? ) => { 2 };
( resource_pack, 0x06 $(,)? ) => { 6 };
( finish_configuration, 0x03 $(,)? ) => { 3 };
( select_known_packs, 0x07 $(,)? ) => { 7 };
( cookie_response, 0x01 $(,)? ) => { 1 };
( pong, 0x05 $(,)? ) => { 5 };
( client_information, 0x00 $(,)? ) => { 0 };
( keep_alive, 0x04 $(,)? ) => { 4 };
}
pub(crate) use prefix_check_config_c2s;

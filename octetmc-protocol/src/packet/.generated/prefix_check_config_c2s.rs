macro_rules! prefix_check_config_c2s {
( client_information, 0x00 $(,)? ) => { 0 };
( custom_click_action, 0x08 $(,)? ) => { 8 };
( resource_pack, 0x06 $(,)? ) => { 6 };
( select_known_packs, 0x07 $(,)? ) => { 7 };
( cookie_response, 0x01 $(,)? ) => { 1 };
( finish_configuration, 0x03 $(,)? ) => { 3 };
( custom_payload, 0x02 $(,)? ) => { 2 };
( keep_alive, 0x04 $(,)? ) => { 4 };
( pong, 0x05 $(,)? ) => { 5 };
}
pub(crate) use prefix_check_config_c2s;

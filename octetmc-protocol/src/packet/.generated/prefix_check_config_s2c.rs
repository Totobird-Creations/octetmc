macro_rules! prefix_check_config_s2c {
( custom_report_details, 0x0F $(,)? ) => { 15 };
( finish_configuration, 0x03 $(,)? ) => { 3 };
( resource_pack_push, 0x09 $(,)? ) => { 9 };
( update_enabled_features, 0x0C $(,)? ) => { 12 };
( disconnect, 0x02 $(,)? ) => { 2 };
( clear_dialog, 0x11 $(,)? ) => { 17 };
( custom_payload, 0x01 $(,)? ) => { 1 };
( registry_data, 0x07 $(,)? ) => { 7 };
( reset_chat, 0x06 $(,)? ) => { 6 };
( update_tags, 0x0D $(,)? ) => { 13 };
( transfer, 0x0B $(,)? ) => { 11 };
( store_cookie, 0x0A $(,)? ) => { 10 };
( select_known_packs, 0x0E $(,)? ) => { 14 };
( resource_pack_pop, 0x08 $(,)? ) => { 8 };
( keep_alive, 0x04 $(,)? ) => { 4 };
( ping, 0x05 $(,)? ) => { 5 };
( show_dialog, 0x12 $(,)? ) => { 18 };
( cookie_request, 0x00 $(,)? ) => { 0 };
( server_links, 0x10 $(,)? ) => { 16 };
}
pub(crate) use prefix_check_config_s2c;

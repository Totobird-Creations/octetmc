macro_rules! prefix_check_config_s2c {
( store_cookie, 0x0A $(,)? ) => { 10 };
( cookie_request, 0x00 $(,)? ) => { 0 };
( custom_report_details, 0x0F $(,)? ) => { 15 };
( update_tags, 0x0D $(,)? ) => { 13 };
( update_enabled_features, 0x0C $(,)? ) => { 12 };
( registry_data, 0x07 $(,)? ) => { 7 };
( transfer, 0x0B $(,)? ) => { 11 };
( disconnect, 0x02 $(,)? ) => { 2 };
( finish_configuration, 0x03 $(,)? ) => { 3 };
( clear_dialog, 0x11 $(,)? ) => { 17 };
( resource_pack_push, 0x09 $(,)? ) => { 9 };
( keep_alive, 0x04 $(,)? ) => { 4 };
( resource_pack_pop, 0x08 $(,)? ) => { 8 };
( custom_payload, 0x01 $(,)? ) => { 1 };
( ping, 0x05 $(,)? ) => { 5 };
( reset_chat, 0x06 $(,)? ) => { 6 };
( select_known_packs, 0x0E $(,)? ) => { 14 };
( server_links, 0x10 $(,)? ) => { 16 };
( show_dialog, 0x12 $(,)? ) => { 18 };
}
pub(crate) use prefix_check_config_s2c;

macro_rules! prefix_check_login_s2c {
( login_disconnect, 0x00 $(,)? ) => { 0 };
( cookie_request, 0x05 $(,)? ) => { 5 };
( login_finished, 0x02 $(,)? ) => { 2 };
( login_compression, 0x03 $(,)? ) => { 3 };
( hello, 0x01 $(,)? ) => { 1 };
( custom_query, 0x04 $(,)? ) => { 4 };
}
pub(crate) use prefix_check_login_s2c;

macro_rules! prefix_check_login_s2c {
( login_compression, 0x03 $(,)? ) => { 3 };
( cookie_request, 0x05 $(,)? ) => { 5 };
( custom_query, 0x04 $(,)? ) => { 4 };
( hello, 0x01 $(,)? ) => { 1 };
( login_finished, 0x02 $(,)? ) => { 2 };
( login_disconnect, 0x00 $(,)? ) => { 0 };
}
pub(crate) use prefix_check_login_s2c;

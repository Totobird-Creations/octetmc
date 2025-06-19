macro_rules! prefix_check_login_c2s {
( login_acknowledged, 0x03 $(,)? ) => { 3 };
( cookie_response, 0x04 $(,)? ) => { 4 };
( custom_query_answer, 0x02 $(,)? ) => { 2 };
( hello, 0x00 $(,)? ) => { 0 };
( key, 0x01 $(,)? ) => { 1 };
}
pub(crate) use prefix_check_login_c2s;

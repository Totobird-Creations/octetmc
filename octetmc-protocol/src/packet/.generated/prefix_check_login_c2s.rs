macro_rules! prefix_check_login_c2s {
( cookie_response, 0x04 $(,)? ) => { 4 };
( hello, 0x00 $(,)? ) => { 0 };
( key, 0x01 $(,)? ) => { 1 };
( login_acknowledged, 0x03 $(,)? ) => { 3 };
( custom_query_answer, 0x02 $(,)? ) => { 2 };
}
pub(crate) use prefix_check_login_c2s;

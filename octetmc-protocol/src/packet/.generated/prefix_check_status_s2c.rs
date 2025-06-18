macro_rules! prefix_check_status_s2c {
( status_response, 0x00 $(,)? ) => { 0 };
( pong_response, 0x01 $(,)? ) => { 1 };
}
pub(crate) use prefix_check_status_s2c;

macro_rules! prefix_check_status_s2c {
( pong_response, 0x01 $(,)? ) => { 1 };
( status_response, 0x00 $(,)? ) => { 0 };
}
pub(crate) use prefix_check_status_s2c;

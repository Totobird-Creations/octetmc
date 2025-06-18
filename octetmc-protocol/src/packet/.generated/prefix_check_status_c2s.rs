macro_rules! prefix_check_status_c2s {
( ping_request, 0x01 $(,)? ) => { 1 };
( status_request, 0x00 $(,)? ) => { 0 };
}
pub(crate) use prefix_check_status_c2s;

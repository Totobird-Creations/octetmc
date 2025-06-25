macro_rules! prefix_check_status_c2s {
( status_request, 0x00 $(,)? ) => { 0 };
( ping_request, 0x01 $(,)? ) => { 1 };
}
pub(crate) use prefix_check_status_c2s;

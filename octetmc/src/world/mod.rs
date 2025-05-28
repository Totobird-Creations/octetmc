//! World information and imports.


use crate::macros::deref_single;
use core::num::NonZeroU8;


deref_single!{
    /// The server's maximum render distance.
    ///
    /// Vanilla clients support values up to 32. This can be set higher, but the client will clamp it.
    pub struct MaxViewDistance(NonZeroU8);
}

//! World information and imports.


use crate::util::macros::deref_single;
use core::num::NonZeroU8;
use bevy_ecs::resource::Resource;


pub mod dimension;


pub(crate) const DEFAULT_VIEW_DISTANCE : NonZeroU8 = unsafe { NonZeroU8::new_unchecked(8) };


deref_single!{
    /// The server's maximum render distance.
    ///
    /// Vanilla clients support values up to 32. This can be set higher, but the client will clamp it.
    #[derive(Resource)]
    pub struct MaxViewDistance(NonZeroU8);
}

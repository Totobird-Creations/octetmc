//! World information and imports.


use crate::util::macros::deref_single;
use octetmc_protocol::value::chunk_pos::ChunkPos;
use core::num::NonZeroU8;
use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;


pub mod dimension;

pub mod generator;


pub(crate) const DEFAULT_VIEW_DISTANCE : NonZeroU8 = unsafe { NonZeroU8::new_unchecked(8) };


deref_single!{
    /// The server's maximum render distance.
    ///
    /// Vanilla clients support values up to 32. This can be set higher, but the client will clamp it.
    #[derive(Resource)]
    pub struct MaxViewDistance(NonZeroU8);
}


deref_single!{
    /// The centre of a client's loaded area.
    #[derive(Component)]
    pub struct ChunkCentre(ChunkPos);
}


deref_single!{
    /// The client's render distance.
    ///
    /// This should not be higher than [`MaxViewDistance`].
    #[derive(Component)]
    pub struct ViewDistance(NonZeroU8);
}

//! 16x16x16 regions in player worlds.


use crate::player::PlayerId;
use crate::util::macros::deref_single;
pub use octetmc_chunksection::{ ChunkSection, ChunkSectionIterator, ChunkSectionEdit };
use octetmc_protocol::value::chunk_section_pos::ChunkSectionPos;
use bevy_ecs::entity::Entity;
use bevy_ecs::component::Component;
use bevy_ecs::bundle::Bundle;


deref_single!{
    /// An [`Entity`] wrapper, intended for [`ChunkSectionId`]s.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Component)]
    pub struct ChunkSectionId(Entity);
    From;
}


/// A Bundle of what a [`ChunkSection`] typically needs.
#[derive(Bundle)]
pub struct ChunkSectionBundle {

    /// The player that this chunk section is tied to.
    pub player  : PlayerId,

    /// The position of this chunk section.
    pub pos     : ChunkSectionPos,

    /// The chunk section itself.
    pub section : ChunkSection

}

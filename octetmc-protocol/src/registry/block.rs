//! Blocks


use crate::value::block_state::BlockState;


include!(".generated/blocks.rs");


/// A block type with states.
pub trait Block
where
    Self       : Into<BlockState> + Default,
    BlockState : From<Self>
{

    /// The default state of this `Block`.
    const DEFAULT_STATE : Self;

}

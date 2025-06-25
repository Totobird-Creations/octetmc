//! Blocks


use crate::value::block_state::BlockState;


include!(".generated/blocks.rs");


/// A block type with properties.
#[expect(private_bounds)]
pub trait Block
where
    Self       : Into<BlockState> + Default + crate::Sealed,
    BlockState : From<Self>
{

    /// The default state of this `Block`.
    const DEFAULT_STATE : Self;

}


/// A block property.
#[expect(private_bounds)]
pub trait BlockProperty : crate::Sealed { }

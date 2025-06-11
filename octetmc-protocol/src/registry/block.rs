//! Blocks


use crate::value::block_state::BlockState;


include!(".generated/blocks.rs");


/// A block type with states.
pub trait Block : Into<BlockState> {
}

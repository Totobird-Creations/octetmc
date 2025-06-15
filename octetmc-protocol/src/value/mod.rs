//! Value types used throughout the game protocol.


pub use uuid;


pub mod varint;

pub mod ident;

pub mod text;

pub mod nbt;

pub mod block_pos;
pub mod dim_block_pos;
pub mod chunk_pos;
pub mod chunk_section_pos;
pub mod character_pos;

pub mod rgb;


pub mod block_state;

pub mod item_slot;

pub mod sound_event;

pub mod particle;


pub mod profile;

pub mod game_mode;


pub mod client_info;

pub mod channel_data;

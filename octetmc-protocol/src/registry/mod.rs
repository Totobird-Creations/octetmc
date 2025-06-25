//! Registry entry values.


pub mod simple_variant;

pub mod cat_variant;

pub mod chicken_variant;

pub mod cow_variant;

pub mod damage_type;

pub mod dimension_type;

pub mod frog_variant;

pub mod painting_variant;

pub mod pig_variant;

pub mod wolf_sound_variant;

pub mod wolf_variant;

pub mod worldgen;


/// A registry type.
#[expect(private_bounds)]
pub trait RegistryType : crate::Sealed { }

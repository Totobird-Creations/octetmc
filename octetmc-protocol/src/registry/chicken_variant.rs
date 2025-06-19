//! `minecraft:chicken_variant` registry.


use crate::mapping_check;
use super::simple_variant::{ SimpleVariant, simple_variant };
use crate::value::ident::ident;
use std::borrow::Cow;


mapping_check!("net.minecraft.world.entity.animal.ChickenVariant", "08317d1203cd9a1709adf5fb2daa8a659d2aff599b0ab28dc5daf8064c3fd8da");


simple_variant!{ ChickenVariant, "chicken variant", "chicken_variant" }

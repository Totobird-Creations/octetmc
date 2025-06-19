//! `minecraft:frog_variant` registry.


use crate::mapping_check;
use super::simple_variant::{ SimpleVariant, simple_variant };
use crate::value::ident::ident;


mapping_check!("net.minecraft.world.entity.animal.frog.FrogVariant", "e46f7ff8ae0682b5226ed2ba54abd8291d82097a967f5607f3b1f123bab733f3");


simple_variant!{ FrogVariant, "frog variant", "frog_variant" }

//! `minecraft:cat_variant` registry.


use crate::mapping_check;
use super::simple_variant::{ SimpleVariant, simple_variant };
use crate::value::ident::ident;


mapping_check!("net.minecraft.world.entity.animal.CatVariant", "fd1fcbb758cc4138be455ab9e32f798137f5d8060e16a5b4ca7162cc88f1bfa9");


simple_variant!{ CatVariant, "cat variant", "cat_variant" }

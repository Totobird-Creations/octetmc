//! `minecraft:cow_variant` registry.


use crate::mapping_check;
use super::simple_variant::{ SimpleVariant, simple_variant };
use crate::value::ident::ident;
use std::borrow::Cow;


mapping_check!("net.minecraft.world.entity.animal.CowVariant", "87af26f88ad5987577d87f9c8c33fa88472c1ea6dba80d6f4241349ccaf96cf2");


simple_variant!{ CowVariant, "cow variant", "cow_variant" }

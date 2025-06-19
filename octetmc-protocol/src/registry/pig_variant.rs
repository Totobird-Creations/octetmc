//! `minecraft:pig_variant` registry.


use crate::mapping_check;
use super::simple_variant::{ SimpleVariant, simple_variant };
use crate::value::ident::ident;
use std::borrow::Cow;


mapping_check!("net.minecraft.world.entity.animal.PigVariant", "78bac02deacb3706b57b65cec60f330e82558060a859eee1570ba87acd3f7308");


simple_variant!{ PigVariant, "pig variant", "pig_variant" }

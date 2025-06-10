//! `minecraft:pig_variant` registry.


use super::simple_variant::{ SimpleVariant, simple_variant };
use crate::value::ident::ident;
use std::borrow::Cow;


simple_variant!{ PigVariant, "pig variant", "pig_variant" }

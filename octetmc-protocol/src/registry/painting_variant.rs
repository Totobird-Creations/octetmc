//! `minecraft:painting_variant` registry.


use crate::mapping_check;
use crate::value::ident::{ Ident, ident };
use crate::value::nbt::{ Nbt, NbtCompound, NbtCompoundEntry, NbtElement };
use crate::packet::config::s2c::registry_data::RegistryEntry;
use core::num::NonZeroU8;
use std::borrow::Cow;


mapping_check!("net.minecraft.world.entity.decoration.PaintingVariant", "649892830b87670f0a4a6b88e752863b5b0a2054d0c56a7834d72f3cc774c339");


/// A painting variant registry entry.
#[derive(Clone, Debug)]
pub struct PaintingVariant<'l> {
    /// The ID of this entry.
    pub id       : Ident<'l>,
    /// The texture of the painting.
    pub asset_id : Ident<'l>,
    /// The width of the painting in blocks.
    pub width    : NonZeroU8,
    /// The height of the painting in blocks.
    pub height   : NonZeroU8
}

include!(".generated/data/painting_variant.rs");


impl PaintingVariant<'_> {

    /// Convert the inner parts of this `PaintingVariant` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `PaintingVariant<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> PaintingVariant<'static> {
        PaintingVariant {
            id       : self.id.into_static_owned(),
            asset_id : self.asset_id.into_static_owned(),
            width    : self.width,
            height   : self.height
        }
    }

    /// Convert the inner parts of this `PaintingVariant` to their owned counterparts.
    ///  Returns the newly created `PaintingVariant<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> PaintingVariant<'static> {
        PaintingVariant {
            id       : self.id.to_static_owned(),
            asset_id : self.asset_id.to_static_owned(),
            width    : self.width,
            height   : self.height
        }
    }

}


impl PaintingVariant<'_> {

    /// Convert `self` to a [`RegistryEntry`].
    pub fn to_registry_entry(&self) -> RegistryEntry {
        RegistryEntry {
            id   : self.id.as_ref(),
            data : Some(Nbt::from(NbtCompound { entries : Cow::Owned(vec![
                NbtCompoundEntry {
                    key   : Cow::Borrowed("asset_id"),
                    value : NbtElement::String(self.asset_id.to_str()),
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("width"),
                    value : NbtElement::Int(self.width.get() as i32)
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("height"),
                    value : NbtElement::Int(self.height.get() as i32)
                }
            ]) })),
        }
    }

}


impl crate::Sealed for PaintingVariant<'_> { }

impl super::RegistryType for PaintingVariant<'_> { }

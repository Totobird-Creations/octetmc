use crate::value::ident::Ident;
use crate::value::nbt::{ Nbt, NbtCompound, NbtCompoundEntry, NbtElement };
use crate::packet::config::s2c::registry_data::RegistryEntry;
use std::borrow::Cow;


/// A painting variant registry entry.
#[derive(Clone, Debug)]
pub struct PaintingVariant<'l> {
    /// The ID of this entry.
    pub id       : Ident<'l>,
    /// The texture of the painting.
    pub asset_id : Ident<'l>,
    /// The width of the painting in blocks.
    pub width    : u8,
    /// The height of the painting in blocks.
    pub height   : u8
}


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
                    value : NbtElement::Int(self.width as i32)
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("height"),
                    value : NbtElement::Int(self.height as i32)
                }
            ]) })),
        }
    }

}

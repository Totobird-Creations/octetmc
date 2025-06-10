use crate::value::ident::Ident;
use crate::value::nbt::{ Nbt, NbtCompound, NbtCompoundEntry, NbtElement };
use crate::packet::config::s2c::registry_data::RegistryEntry;
use std::borrow::Cow;


/// A simple variant registry entry.
#[derive(Clone, Debug)]
pub struct SimpleVariant<'l> {
    /// The ID of this entry.
    pub id       : Ident<'l>,
    /// The resource ID of the asset.
    pub asset_id : Ident<'l>
}


impl SimpleVariant<'_> {

    /// Convert the inner parts of this `SimpleVariant` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `SimpleVariant<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> SimpleVariant<'static> {
        SimpleVariant {
            id       : self.id.into_static_owned(),
            asset_id : self.asset_id.into_static_owned()
        }
    }

    /// Convert the inner parts of this `SimpleVariant` to their owned counterparts.
    ///  Returns the newly created `SimpleVariant<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> SimpleVariant<'static> {
        SimpleVariant {
            id       : self.id.to_static_owned(),
            asset_id : self.asset_id.to_static_owned()
        }
    }

}


impl SimpleVariant<'_> {

    /// Convert `self` to a [`RegistryEntry`].
    pub fn to_registry_entry(&self) -> RegistryEntry {
        RegistryEntry {
            id   : self.id.as_ref(),
            data : Some(Nbt::from(NbtCompound { entries : Cow::Owned(vec![
                NbtCompoundEntry {
                    key   : Cow::Borrowed("asset_id"),
                    value : NbtElement::String(self.asset_id.to_str()),
                }
            ]) })),
        }
    }

}

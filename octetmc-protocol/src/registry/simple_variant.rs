use crate::value::ident::Ident;
use crate::value::nbt::{ Nbt, NbtCompound, NbtCompoundEntry, NbtElement };
use crate::packet::config::s2c::registry_data::RegistryEntry;
use std::borrow::Cow;


/// A simple variant registry entry.
#[derive(Clone, Debug)]
pub struct SimpleVariant<'l> {
    /// The ID of this entry.
    pub id       : Ident<'l>,
    /// The model of this entry.
    pub model    : Option<Cow<'l, str>>,
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
            model    : self.model.map(|model| Cow::Owned(model.into_owned())),
            asset_id : self.asset_id.into_static_owned()
        }
    }

    /// Convert the inner parts of this `SimpleVariant` to their owned counterparts.
    ///  Returns the newly created `SimpleVariant<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> SimpleVariant<'static> {
        SimpleVariant {
            id       : self.id.to_static_owned(),
            model    : self.model.as_ref().map(|model| Cow::Owned((**model).to_owned())),
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


macro_rules! simple_variant { ( $ident:ident , $name:tt , $id:tt ) => {

    #[doc = concat!("A ", $name, " registry entry.")]
    #[derive(Clone, Debug)]
    pub struct $ident <'l> ( pub $crate::registry::SimpleVariant<'l> );

    include!(concat!(".generated/data/", $id, ".rs"));

    impl<'l> core::ops::Deref for $ident<'l> {
        type Target = $crate::registry::SimpleVariant<'l>;
        fn deref(&self) -> &Self::Target { &self.0 }
    }

    impl<'l> core::ops::DerefMut for $ident<'l> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }

} }
pub(super) use simple_variant;

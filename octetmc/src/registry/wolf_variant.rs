use octetmc_protocol::value::ident::Ident;
use octetmc_protocol::value::nbt::{ Nbt, NbtCompound, NbtCompoundEntry, NbtElement };
use octetmc_protocol::packet::config::s2c::registry_data::RegistryEntry;
use std::borrow::Cow;


/// A wolf variant registry entry.
#[derive(Clone, Debug)]
pub struct WolfVariant<'l> {
    /// The ID of this entry.
    pub id             : Ident<'l>,
    /// Texture for wild wolves.
    pub wild_asset_id  : Ident<'l>,
    /// Texture for tamed wolves.
    pub tame_asset_id  : Ident<'l>,
    /// Texture for angry wolves.
    pub angry_asset_id : Ident<'l>
}

impl WolfVariant<'_> {

    /// A wolf variant for use in minimal registries.
    pub const MINIMAL : WolfVariant<'static> = WolfVariant {
        id             : Ident::new("octetmc", "empty"),
        wild_asset_id  : Ident::new("octetmc", "empty"),
        tame_asset_id  : Ident::new("octetmc", "empty"),
        angry_asset_id : Ident::new("octetmc", "empty")
    };

}


impl WolfVariant<'_> {

    /// Convert the inner parts of this `WolfVariant` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `WolfVariant<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> WolfVariant<'static> {
        WolfVariant {
            id             : self.id.into_static_owned(),
            wild_asset_id  : self.wild_asset_id.into_static_owned(),
            tame_asset_id  : self.tame_asset_id.into_static_owned(),
            angry_asset_id : self.angry_asset_id.into_static_owned()
        }
    }

    /// Convert the inner parts of this `WolfVariant` to their owned counterparts.
    ///  Returns the newly created `WolfVariant<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> WolfVariant<'static> {
        WolfVariant {
            id             : self.id.to_static_owned(),
            wild_asset_id  : self.wild_asset_id.to_static_owned(),
            tame_asset_id  : self.tame_asset_id.to_static_owned(),
            angry_asset_id : self.angry_asset_id.to_static_owned()
        }
    }

}


impl WolfVariant<'_> {

    /// Convert `self` to a [`RegistryEntry`].
    pub fn to_registry_entry(&self) -> RegistryEntry {
        RegistryEntry {
            id   : self.id.as_ref(),
            data : Some(Nbt::from(NbtCompound { entries : Cow::Owned(vec![
                NbtCompoundEntry {
                    key   : Cow::Borrowed("assets"),
                    value : NbtElement::Compound(NbtCompound { entries : Cow::Owned(vec![
                        NbtCompoundEntry {
                            key   : Cow::Borrowed("wild"),
                            value : NbtElement::String(Cow::Owned(self.wild_asset_id.to_string())),
                        },
                        NbtCompoundEntry {
                            key   : Cow::Borrowed("tame"),
                            value : NbtElement::String(Cow::Owned(self.tame_asset_id.to_string())),
                        },
                        NbtCompoundEntry {
                            key   : Cow::Borrowed("angry"),
                            value : NbtElement::String(Cow::Owned(self.angry_asset_id.to_string())),
                        }
                    ]) })
                }
            ]) })),
        }
    }

}

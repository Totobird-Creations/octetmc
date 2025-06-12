//! `minecraft:wolf_sound_variant` registry.


use crate::value::ident::{ Ident, ident };
use crate::value::nbt::{ Nbt, NbtCompound, NbtCompoundEntry, NbtElement };
use crate::packet::config::s2c::registry_data::RegistryEntry;
use std::borrow::Cow;


/// A wolf sound variant registry entry.
#[derive(Clone, Debug)]
pub struct WolfSoundVariant<'l> {
    /// The ID of this entry.
    pub id            : Ident<'l>,
    /// Hurt sound for wolves.
    pub hurt_sound    : Ident<'l>,
    /// Sound for wolves when panting.
    pub pant_sound    : Ident<'l>,
    /// Sound for wolves when whining.
    pub whine_sound   : Ident<'l>,
    /// Ambient sound for wolves.
    pub ambient_sound : Ident<'l>,
    /// Death sound for wolves.
    pub death_sound   : Ident<'l>,
    /// Sound for wolves when growling.
    pub growl_sound   : Ident<'l>
}

include!(".generated/data/wolf_sound_variant.rs");


impl WolfSoundVariant<'_> {

    /// Convert the inner parts of this `WolfSoundVariant` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `WolfSoundVariant<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> WolfSoundVariant<'static> {
        WolfSoundVariant {
            id            : self.id.into_static_owned(),
            hurt_sound    : self.hurt_sound.into_static_owned(),
            pant_sound    : self.pant_sound.into_static_owned(),
            whine_sound   : self.whine_sound.into_static_owned(),
            ambient_sound : self.ambient_sound.into_static_owned(),
            death_sound   : self.death_sound.into_static_owned(),
            growl_sound   : self.growl_sound.into_static_owned()
        }
    }

    /// Convert the inner parts of this `WolfSoundVariant` to their owned counterparts.
    ///  Returns the newly created `WolfSoundVariant<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> WolfSoundVariant<'static> {
        WolfSoundVariant {
            id            : self.id.to_static_owned(),
            hurt_sound    : self.hurt_sound.to_static_owned(),
            pant_sound    : self.pant_sound.to_static_owned(),
            whine_sound   : self.whine_sound.to_static_owned(),
            ambient_sound : self.ambient_sound.to_static_owned(),
            death_sound   : self.death_sound.to_static_owned(),
            growl_sound   : self.growl_sound.to_static_owned()
        }
    }

}


impl WolfSoundVariant<'_> {

    /// Convert `self` to a [`RegistryEntry`].
    pub fn to_registry_entry(&self) -> RegistryEntry {
        RegistryEntry {
            id   : self.id.as_ref(),
            data : Some(Nbt::from(NbtCompound { entries : Cow::Owned(vec![
                NbtCompoundEntry {
                    key   : Cow::Borrowed("hurt_sound"),
                    value : NbtElement::String(self.hurt_sound.to_str()),
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("pant_sound"),
                    value : NbtElement::String(self.pant_sound.to_str()),
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("whine_sound"),
                    value : NbtElement::String(self.whine_sound.to_str()),
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("ambient_sound"),
                    value : NbtElement::String(self.ambient_sound.to_str()),
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("death_sound"),
                    value : NbtElement::String(self.death_sound.to_str()),
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("growl_sound"),
                    value : NbtElement::String(self.growl_sound.to_str()),
                }
            ]) })),
        }
    }

}


impl crate::Sealed for WolfSoundVariant<'_> { }

impl super::RegistryType for WolfSoundVariant<'_> { }

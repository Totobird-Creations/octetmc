use crate::value::ident::Ident;
use crate::value::nbt::{ Nbt, NbtCompound, NbtCompoundEntry, NbtElement };
use crate::packet::config::s2c::registry_data::RegistryEntry;
use std::borrow::Cow;


/// A painting variant registry entry.
#[derive(Clone, Debug)]
pub struct DamageType<'l> {
    /// The ID of this entry.
    pub id           : Ident<'l>,
    /// The ID of the death message.
    ///
    /// `death.attack.<message_id>`
    pub message_id   : Cow<'l, str>,
    /// Effect played when the player suffers this damage,
    ///  including the sound that is played.
    pub effects      : DamageEffects,
    /// Defines how the death message is constructed.
    pub message_type : DeathMessageType
}

include!(".generated/data/damage_type.rs");


impl DamageType<'_> {

    /// Convert the inner parts of this `DamageType` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `DamageType<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> DamageType<'static> {
        DamageType {
            id           : self.id.into_static_owned(),
            message_id   : Cow::Owned(self.message_id.into_owned()),
            effects      : self.effects,
            message_type : self.message_type
        }
    }

    /// Convert the inner parts of this `DamageType` to their owned counterparts.
    ///  Returns the newly created `DamageType<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> DamageType<'static> {
        DamageType {
            id           : self.id.to_static_owned(),
            message_id   : Cow::Owned((*self.message_id).to_owned()),
            effects      : self.effects,
            message_type : self.message_type
        }
    }

}


/// Effect played when the player suffers some damage,
///  including the sound that is played.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DamageEffects {
    /// Hurt (Generic)
    Hurt,
    /// Thorns (Thorns enchantment)
    Thorns,
    /// Drowning (Underwater)
    Drowning,
    /// Burning (Fire, Lava, Magma block)
    Burning,
    /// Poking (Berry bush)
    Poking,
    /// Freezing (Powder snow)
    Freezing
}

/// How a death message is constructed.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DeathMessageType {
    /// Message built normally.
    Default,
    /// Most significant fall damage to be considered.
    FallVariants,
    /// String-interpolates [MCPE-28723](https://bugs.mojang.com/browse/MCPE-28723) into the message.
    IntentionalGameDesign
}


impl DamageType<'_> {

    /// Convert `self` to a [`RegistryEntry`].
    pub fn to_registry_entry(&self) -> RegistryEntry {
        RegistryEntry {
            id   : self.id.as_ref(),
            data : Some(Nbt::from(NbtCompound { entries : Cow::Owned(vec![
                NbtCompoundEntry {
                    key   : Cow::Borrowed("message_id"),
                    value : NbtElement::String(Cow::Borrowed(&self.message_id)),
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("scaling"),
                    value : NbtElement::String(Cow::Borrowed("never"))
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("exhaustion"),
                    value : NbtElement::Float(0.0)
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("effects"),
                    value : NbtElement::String(Cow::Borrowed(match (self.effects) {
                        DamageEffects::Hurt     => "hurt",
                        DamageEffects::Thorns   => "thorns",
                        DamageEffects::Drowning => "drowning",
                        DamageEffects::Burning  => "burning",
                        DamageEffects::Poking   => "poking",
                        DamageEffects::Freezing => "freezing"
                    }))
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("death_message_type"),
                    value : NbtElement::String(Cow::Borrowed(match (self.message_type) {
                        DeathMessageType::Default               => "default",
                        DeathMessageType::FallVariants          => "fall_variants",
                        DeathMessageType::IntentionalGameDesign => "intentional_game_design"
                    }))
                }
            ]) })),
        }
    }

}

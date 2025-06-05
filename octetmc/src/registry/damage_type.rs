use octetmc_protocol::value::ident::Ident;
use octetmc_protocol::value::nbt::{ Nbt, NbtCompound, NbtCompoundEntry, NbtElement };
use octetmc_protocol::packet::config::s2c::registry_data::RegistryEntry;
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
    pub effects      : Option<DamageEffects>,
    /// Defines how the death message is constructed.
    pub message_type : Option<DeathMessageType>
}

impl DamageType<'_> {

    /// Vanilla damage types required to connect a client to the server.
    pub const VANILLA_DAMAGE_TYPES : &'static [DamageType<'static>] = &[
        Self::ON_FIRE,
        Self::IN_FIRE,
        Self::CAMPFIRE,
        Self::LIGHTNING_BOLT,
        Self::LAVA,
        Self::HOT_FLOOR,
        Self::IN_WALL,
        Self::CRAMMING,
        Self::DROWN,
        Self::STARVE,
        Self::CACTUS,
        Self::FALL,
        Self::ENDER_PEARL,
        Self::FLY_INTO_WALL,
        Self::OUT_OF_WORLD,
        Self::GENERIC
    ];

    /// Vanilla `minecraft:on_fire` damage type.
    pub const ON_FIRE : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("on_fire"),
        message_id   : Cow::Borrowed("onFire"),
        effects      : Some(DamageEffects::Burning),
        message_type : None
    };

    /// Vanilla `minecraft:in_fire` damage type.
    pub const IN_FIRE : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("in_fire"),
        message_id   : Cow::Borrowed("inFire"),
        effects      : Some(DamageEffects::Burning),
        message_type : None
    };

    /// Vanilla `minecraft:campfire` damage type.
    pub const CAMPFIRE : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("campfire"),
        message_id   : Cow::Borrowed("inFire"),
        effects      : Some(DamageEffects::Burning),
        message_type : None
    };

    /// Vanilla `minecraft:lightning_bolt` damage type.
    pub const LIGHTNING_BOLT : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("lightning_bolt"),
        message_id   : Cow::Borrowed("lightningBolt"),
        effects      : None,
        message_type : None
    };

    /// Vanilla `minecraft:lava` damage type.
    pub const LAVA : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("lava"),
        message_id   : Cow::Borrowed("lava"),
        effects      : Some(DamageEffects::Burning),
        message_type : None
    };

    /// Vanilla `minecraft:hot_floor` damage type.
    pub const HOT_FLOOR : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("hot_floor"),
        message_id   : Cow::Borrowed("hotFloor"),
        effects      : Some(DamageEffects::Burning),
        message_type : None
    };

    /// Vanilla `minecraft:in_wall` damage type.
    pub const IN_WALL : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("in_wall"),
        message_id   : Cow::Borrowed("inWall"),
        effects      : None,
        message_type : None
    };

    /// Vanilla `minecraft:cramming` damage type.
    pub const CRAMMING : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("cramming"),
        message_id   : Cow::Borrowed("cramming"),
        effects      : None,
        message_type : None
    };

    /// Vanilla `minecraft:drown` damage type.
    pub const DROWN : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("drown"),
        message_id   : Cow::Borrowed("drown"),
        effects      : Some(DamageEffects::Drowning),
        message_type : None
    };

    /// Vanilla `minecraft:starve` damage type.
    pub const STARVE : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("starve"),
        message_id   : Cow::Borrowed("starve"),
        effects      : None,
        message_type : None
    };

    /// Vanilla `minecraft:cactus` damage type.
    pub const CACTUS : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("cactus"),
        message_id   : Cow::Borrowed("cactus"),
        effects      : None,
        message_type : None
    };

    /// Vanilla `minecraft:fall` damage type.
    pub const FALL : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("fall"),
        message_id   : Cow::Borrowed("fall"),
        effects      : None,
        message_type : Some(DeathMessageType::FallVariants)
    };

    /// Vanilla `minecraft:ender_pearl` damage type.
    pub const ENDER_PEARL : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("ender_pearl"),
        message_id   : Cow::Borrowed("fall"),
        effects      : None,
        message_type : Some(DeathMessageType::FallVariants)
    };

    /// Vanilla `minecraft:fly_into_wall` damage type.
    pub const FLY_INTO_WALL : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("fly_into_wall"),
        message_id   : Cow::Borrowed("flyIntoWall"),
        effects      : None,
        message_type : None
    };

    /// Vanilla `minecraft:out_of_world` damage type.
    pub const OUT_OF_WORLD : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("out_of_world"),
        message_id   : Cow::Borrowed("outOfWorld"),
        effects      : None,
        message_type : None
    };

    /// Vanilla `minecraft:generic` damage type.
    pub const GENERIC : DamageType<'static> = DamageType {
        id           : Ident::new_vanilla("generic"),
        message_id   : Cow::Borrowed("generic"),
        effects      : None,
        message_type : None
    };

}


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
        let mut entries = vec![
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
            }
        ];
        if let Some(effects) = self.effects {
            entries.push(NbtCompoundEntry {
                key   : Cow::Borrowed("effects"),
                value : NbtElement::String(Cow::Borrowed(match (effects) {
                    DamageEffects::Hurt     => "hurt",
                    DamageEffects::Thorns   => "thorns",
                    DamageEffects::Drowning => "drowning",
                    DamageEffects::Burning  => "burning",
                    DamageEffects::Poking   => "poking",
                    DamageEffects::Freezing => "freezing"
                }))
            });
        }
        if let Some(message_type) = self.message_type {
            entries.push(NbtCompoundEntry {
                key   : Cow::Borrowed("death_message_type"),
                value : NbtElement::String(Cow::Borrowed(match (message_type) {
                    DeathMessageType::Default               => "default",
                    DeathMessageType::FallVariants          => "fall_variants",
                    DeathMessageType::IntentionalGameDesign => "intentional_game_design"
                }))
            });
        }
        RegistryEntry {
            id   : self.id.as_ref(),
            data : Some(Nbt::from(NbtCompound { entries : Cow::Owned(entries) })),
        }
    }

}

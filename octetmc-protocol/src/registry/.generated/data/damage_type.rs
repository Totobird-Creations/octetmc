impl DamageType<'_> {

    /// Vanilla `minecraft:sonic_boom` damage type.
    pub const SONIC_BOOM : DamageType<'static> = DamageType {
        id           : ident![sonic_boom],
        message_id   : Cow::Borrowed("sonic_boom"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:mob_projectile` damage type.
    pub const MOB_PROJECTILE : DamageType<'static> = DamageType {
        id           : ident![mob_projectile],
        message_id   : Cow::Borrowed("mob"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:hot_floor` damage type.
    pub const HOT_FLOOR : DamageType<'static> = DamageType {
        id           : ident![hot_floor],
        message_id   : Cow::Borrowed("hotFloor"),
        effects      : DamageEffects::Burning,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:magic` damage type.
    pub const MAGIC : DamageType<'static> = DamageType {
        id           : ident![magic],
        message_id   : Cow::Borrowed("magic"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:generic_kill` damage type.
    pub const GENERIC_KILL : DamageType<'static> = DamageType {
        id           : ident![generic_kill],
        message_id   : Cow::Borrowed("genericKill"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:wither` damage type.
    pub const WITHER : DamageType<'static> = DamageType {
        id           : ident![wither],
        message_id   : Cow::Borrowed("wither"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:drown` damage type.
    pub const DROWN : DamageType<'static> = DamageType {
        id           : ident![drown],
        message_id   : Cow::Borrowed("drown"),
        effects      : DamageEffects::Drowning,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:mob_attack` damage type.
    pub const MOB_ATTACK : DamageType<'static> = DamageType {
        id           : ident![mob_attack],
        message_id   : Cow::Borrowed("mob"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:in_fire` damage type.
    pub const IN_FIRE : DamageType<'static> = DamageType {
        id           : ident![in_fire],
        message_id   : Cow::Borrowed("inFire"),
        effects      : DamageEffects::Burning,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:dry_out` damage type.
    pub const DRY_OUT : DamageType<'static> = DamageType {
        id           : ident![dry_out],
        message_id   : Cow::Borrowed("dryout"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:player_explosion` damage type.
    pub const PLAYER_EXPLOSION : DamageType<'static> = DamageType {
        id           : ident![player_explosion],
        message_id   : Cow::Borrowed("explosion.player"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:bad_respawn_point` damage type.
    pub const BAD_RESPAWN_POINT : DamageType<'static> = DamageType {
        id           : ident![bad_respawn_point],
        message_id   : Cow::Borrowed("badRespawnPoint"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::IntentionalGameDesign
    };

    /// Vanilla `minecraft:fly_into_wall` damage type.
    pub const FLY_INTO_WALL : DamageType<'static> = DamageType {
        id           : ident![fly_into_wall],
        message_id   : Cow::Borrowed("flyIntoWall"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:sweet_berry_bush` damage type.
    pub const SWEET_BERRY_BUSH : DamageType<'static> = DamageType {
        id           : ident![sweet_berry_bush],
        message_id   : Cow::Borrowed("sweetBerryBush"),
        effects      : DamageEffects::Poking,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:lava` damage type.
    pub const LAVA : DamageType<'static> = DamageType {
        id           : ident![lava],
        message_id   : Cow::Borrowed("lava"),
        effects      : DamageEffects::Burning,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:thrown` damage type.
    pub const THROWN : DamageType<'static> = DamageType {
        id           : ident![thrown],
        message_id   : Cow::Borrowed("thrown"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:fireworks` damage type.
    pub const FIREWORKS : DamageType<'static> = DamageType {
        id           : ident![fireworks],
        message_id   : Cow::Borrowed("fireworks"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:fireball` damage type.
    pub const FIREBALL : DamageType<'static> = DamageType {
        id           : ident![fireball],
        message_id   : Cow::Borrowed("fireball"),
        effects      : DamageEffects::Burning,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:trident` damage type.
    pub const TRIDENT : DamageType<'static> = DamageType {
        id           : ident![trident],
        message_id   : Cow::Borrowed("trident"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:sting` damage type.
    pub const STING : DamageType<'static> = DamageType {
        id           : ident![sting],
        message_id   : Cow::Borrowed("sting"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:explosion` damage type.
    pub const EXPLOSION : DamageType<'static> = DamageType {
        id           : ident![explosion],
        message_id   : Cow::Borrowed("explosion"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:lightning_bolt` damage type.
    pub const LIGHTNING_BOLT : DamageType<'static> = DamageType {
        id           : ident![lightning_bolt],
        message_id   : Cow::Borrowed("lightningBolt"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:generic` damage type.
    pub const GENERIC : DamageType<'static> = DamageType {
        id           : ident![generic],
        message_id   : Cow::Borrowed("generic"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:mace_smash` damage type.
    pub const MACE_SMASH : DamageType<'static> = DamageType {
        id           : ident![mace_smash],
        message_id   : Cow::Borrowed("mace_smash"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:dragon_breath` damage type.
    pub const DRAGON_BREATH : DamageType<'static> = DamageType {
        id           : ident![dragon_breath],
        message_id   : Cow::Borrowed("dragonBreath"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:freeze` damage type.
    pub const FREEZE : DamageType<'static> = DamageType {
        id           : ident![freeze],
        message_id   : Cow::Borrowed("freeze"),
        effects      : DamageEffects::Freezing,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:cactus` damage type.
    pub const CACTUS : DamageType<'static> = DamageType {
        id           : ident![cactus],
        message_id   : Cow::Borrowed("cactus"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:stalagmite` damage type.
    pub const STALAGMITE : DamageType<'static> = DamageType {
        id           : ident![stalagmite],
        message_id   : Cow::Borrowed("stalagmite"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:fall` damage type.
    pub const FALL : DamageType<'static> = DamageType {
        id           : ident![fall],
        message_id   : Cow::Borrowed("fall"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::FallVariants
    };

    /// Vanilla `minecraft:ender_pearl` damage type.
    pub const ENDER_PEARL : DamageType<'static> = DamageType {
        id           : ident![ender_pearl],
        message_id   : Cow::Borrowed("fall"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::FallVariants
    };

    /// Vanilla `minecraft:out_of_world` damage type.
    pub const OUT_OF_WORLD : DamageType<'static> = DamageType {
        id           : ident![out_of_world],
        message_id   : Cow::Borrowed("outOfWorld"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:cramming` damage type.
    pub const CRAMMING : DamageType<'static> = DamageType {
        id           : ident![cramming],
        message_id   : Cow::Borrowed("cramming"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:falling_anvil` damage type.
    pub const FALLING_ANVIL : DamageType<'static> = DamageType {
        id           : ident![falling_anvil],
        message_id   : Cow::Borrowed("anvil"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:falling_block` damage type.
    pub const FALLING_BLOCK : DamageType<'static> = DamageType {
        id           : ident![falling_block],
        message_id   : Cow::Borrowed("fallingBlock"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:thorns` damage type.
    pub const THORNS : DamageType<'static> = DamageType {
        id           : ident![thorns],
        message_id   : Cow::Borrowed("thorns"),
        effects      : DamageEffects::Thorns,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:indirect_magic` damage type.
    pub const INDIRECT_MAGIC : DamageType<'static> = DamageType {
        id           : ident![indirect_magic],
        message_id   : Cow::Borrowed("indirectMagic"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:starve` damage type.
    pub const STARVE : DamageType<'static> = DamageType {
        id           : ident![starve],
        message_id   : Cow::Borrowed("starve"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:arrow` damage type.
    pub const ARROW : DamageType<'static> = DamageType {
        id           : ident![arrow],
        message_id   : Cow::Borrowed("arrow"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:outside_border` damage type.
    pub const OUTSIDE_BORDER : DamageType<'static> = DamageType {
        id           : ident![outside_border],
        message_id   : Cow::Borrowed("outsideBorder"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:unattributed_fireball` damage type.
    pub const UNATTRIBUTED_FIREBALL : DamageType<'static> = DamageType {
        id           : ident![unattributed_fireball],
        message_id   : Cow::Borrowed("onFire"),
        effects      : DamageEffects::Burning,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:spit` damage type.
    pub const SPIT : DamageType<'static> = DamageType {
        id           : ident![spit],
        message_id   : Cow::Borrowed("mob"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:on_fire` damage type.
    pub const ON_FIRE : DamageType<'static> = DamageType {
        id           : ident![on_fire],
        message_id   : Cow::Borrowed("onFire"),
        effects      : DamageEffects::Burning,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:player_attack` damage type.
    pub const PLAYER_ATTACK : DamageType<'static> = DamageType {
        id           : ident![player_attack],
        message_id   : Cow::Borrowed("player"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:falling_stalactite` damage type.
    pub const FALLING_STALACTITE : DamageType<'static> = DamageType {
        id           : ident![falling_stalactite],
        message_id   : Cow::Borrowed("fallingStalactite"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:in_wall` damage type.
    pub const IN_WALL : DamageType<'static> = DamageType {
        id           : ident![in_wall],
        message_id   : Cow::Borrowed("inWall"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:wind_charge` damage type.
    pub const WIND_CHARGE : DamageType<'static> = DamageType {
        id           : ident![wind_charge],
        message_id   : Cow::Borrowed("mob"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:mob_attack_no_aggro` damage type.
    pub const MOB_ATTACK_NO_AGGRO : DamageType<'static> = DamageType {
        id           : ident![mob_attack_no_aggro],
        message_id   : Cow::Borrowed("mob"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:campfire` damage type.
    pub const CAMPFIRE : DamageType<'static> = DamageType {
        id           : ident![campfire],
        message_id   : Cow::Borrowed("inFire"),
        effects      : DamageEffects::Burning,
        message_type : DeathMessageType::Default
    };

    /// Vanilla `minecraft:wither_skull` damage type.
    pub const WITHER_SKULL : DamageType<'static> = DamageType {
        id           : ident![wither_skull],
        message_id   : Cow::Borrowed("witherSkull"),
        effects      : DamageEffects::Hurt,
        message_type : DeathMessageType::Default
    };

    /// All vanilla damage types.
    pub const VANILLA_DAMAGE_TYPES : &'static [DamageType<'static>] = &[
        Self::SONIC_BOOM,
        Self::MOB_PROJECTILE,
        Self::HOT_FLOOR,
        Self::MAGIC,
        Self::GENERIC_KILL,
        Self::WITHER,
        Self::DROWN,
        Self::MOB_ATTACK,
        Self::IN_FIRE,
        Self::DRY_OUT,
        Self::PLAYER_EXPLOSION,
        Self::BAD_RESPAWN_POINT,
        Self::FLY_INTO_WALL,
        Self::SWEET_BERRY_BUSH,
        Self::LAVA,
        Self::THROWN,
        Self::FIREWORKS,
        Self::FIREBALL,
        Self::TRIDENT,
        Self::STING,
        Self::EXPLOSION,
        Self::LIGHTNING_BOLT,
        Self::GENERIC,
        Self::MACE_SMASH,
        Self::DRAGON_BREATH,
        Self::FREEZE,
        Self::CACTUS,
        Self::STALAGMITE,
        Self::FALL,
        Self::ENDER_PEARL,
        Self::OUT_OF_WORLD,
        Self::CRAMMING,
        Self::FALLING_ANVIL,
        Self::FALLING_BLOCK,
        Self::THORNS,
        Self::INDIRECT_MAGIC,
        Self::STARVE,
        Self::ARROW,
        Self::OUTSIDE_BORDER,
        Self::UNATTRIBUTED_FIREBALL,
        Self::SPIT,
        Self::ON_FIRE,
        Self::PLAYER_ATTACK,
        Self::FALLING_STALACTITE,
        Self::IN_WALL,
        Self::WIND_CHARGE,
        Self::MOB_ATTACK_NO_AGGRO,
        Self::CAMPFIRE,
        Self::WITHER_SKULL,
    ];

}

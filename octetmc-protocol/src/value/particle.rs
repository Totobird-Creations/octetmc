//! Particles


use crate::mapping_check;
use crate::value::item_slot::ItemSlot;
use crate::value::rgb::{ Rgb, Argb };
use crate::value::block_state::BlockState;


mapping_check!("net.minecraft.core.particles.BlockParticleOption", "1e00041411d0aa0e66b5f26a83522f5d063c6c4c3e4d8c8b6eff972e08e8b27e");
mapping_check!("net.minecraft.core.particles.ColorParticleOption", "91a4778557d3707dedc080d84c68ec655adec68aea83868a73796e7c1c240365");
mapping_check!("net.minecraft.core.particles.DustColorTransitionOptions", "34b7bac788b769968a41b8b154df97f943a019d3430242ca72bb086e0e621496");
mapping_check!("net.minecraft.core.particles.DustParticleOptions", "71a13b0c60e1496dab3dfa71bee1b3b509ff053f358ffcdf0cd02d8bb3431252");
mapping_check!("net.minecraft.core.particles.ItemParticleOption", "da92379e8b2394b1238d6a038bed88ed0af066ac60d8323cf450b0c106871c27");
mapping_check!("net.minecraft.core.particles.ParticleOptions", "39b428843fecb3217ac0de048b8d93f53d257453c0a50dc5c70e9aba7c79d337");
mapping_check!("net.minecraft.core.particles.ParticleType", "b8178dcdf80e2b1c1c3390edb48dc533f3ca2b8315b9ba95074b0b69f4558ef5");
mapping_check!("net.minecraft.core.particles.ScalableParticleOptionsBase", "629993d8235d4721aac8fb4453fe38c2a47f5a8dbdb5f51fc2fc5f85bd81488e");
mapping_check!("net.minecraft.core.particles.SculkChargeParticleOptions", "7b791907dcb25c9be2809150ff6635988bb4577873a2573e98907656dee77773");
mapping_check!("net.minecraft.core.particles.ShriekParticleOption", "495906ecc4610cf1433d68644310df00f6b7babf61bb8d84ec350c5f822c6004");
mapping_check!("net.minecraft.core.particles.SimpleParticleType", "da9b8f138d002d66144abbf2fb285bdaebecc249f4a05d50f101bdef97230c02");
mapping_check!("net.minecraft.core.particles.TrailParticleOption", "2aade615d773b9fb85e6c23b818f792fb8876a4ddaf0f946299b91281eb6e9b8");
mapping_check!("net.minecraft.core.particles.VibrationParticleOption", "75c5c948f75f16df86c06da75a14473240895d8302daa7a083eb6f6a94ab695d");


/// Unimplemented partible.
#[derive(Debug, Clone)]
pub enum TODO {}


/// Types of particles.
#[derive(Debug, Clone)]
pub enum Particle<'l> {

    /// Angry Villager
    AngryVillager,

    /// Block
    Block {
        /// The block state.
        state : BlockState
    },

    /// Block Marker
    BlockMarker {
        /// The block state.
        state : BlockState
    },

    /// Bubble
    Bubble,

    /// Cloud
    Cloud,

    /// Crit
    Crit,

    /// Damage Indicator
    DamageIndicator,

    /// Dragon Breath
    DragonBreath,

    /// Dripping Lava
    DrippingLava,

    /// Falling Lava
    FallingLava,

    /// Landing Lava
    LandingLava,

    /// Dripping Water
    DrippingWater,

    /// Falling Water
    FallingWater,

    /// Dust
    Dust {
        /// Colour of the dust.
        colour : Rgb,
        /// Size of the particle.
        scale  : f32
    },

    /// Dust Colour Transition
    DustColourTransition {
        /// Starting colour of the dust.
        from_colour : Rgb,
        /// Final colour of the dust.
        to_colour   : Rgb,
        /// Size of the particle.
        scale       : f32
    },

    /// Effect
    Effect,

    /// Elder guardian
    ElderGuardian,

    /// Enchanted Hit
    EnchantedHit,

    /// Enchant
    Enchant,

    /// End Rod
    EndRod,

    /// Entity Effect
    EntityEffect {
        /// Colour of the spirals.
        colour : Argb
    },

    /// Explosion Emitter
    ExplosionEmitter,

    /// Explosion
    Explosion,

    /// Gust
    Gust,

    /// Small Gust
    SmallGust,

    /// Gust Emitter Large
    GustEmitterLarge,

    /// Gust Emitter Small
    GustEmitterSmall,

    /// Sonic Boom
    SonicBoom,

    /// Falling Dust
    FallingDust {
        /// The block state.
        state : BlockState
    },

    /// Firework
    Firework,

    /// Fishing
    Fishing,

    /// Flame
    Flame,

    /// Infested
    Infested,

    /// Cherry Leaves
    CherryLeaves,

    /// Pale Oak Leaves
    PaleOakLeaves,

    /// Tinted Leaves
    TintedLeaves {
        /// Colour of the leaves.
        colour : Argb
    },

    /// Sculk Soul
    SculkSoul,

    /// Sculk Charge
    SculkCharge {
        /// How much the particle will be rotated when displayed.
        roll : f32
    },

    /// Sculk Charge Pop
    SculkChargePop,

    /// Soul Fire Flame
    SoulFireFlame,

    /// Soul
    Soul,

    /// Flash
    Flash,

    /// Happy Villager
    HappyVillager,

    /// Composter
    Composter,

    /// Heart
    Heart,

    /// Instant Effect
    InstantEffect,

    /// Item
    Item {
        /// The item.
        item : ItemSlot<'l>
    },

    /// Vibration
    Vibration(TODO),

    /// Trail
    Trail {
        /// Target X.
        x        : f64,
        /// Target Y.
        y        : f64,
        /// Target Z.
        z        : f64,
        /// Colour of the trial.
        colour   : Rgb,
        /// Lifetime, in ticks.
        duration : u32
    },

    /// Item Slime
    ItemSlime,

    /// Item Cobweb
    ItemCobweb,

    /// Item Snowball
    ItemSnowball,

    /// Large Smoke
    LargeSmoke,

    /// Lava
    Lava,

    /// Mycelium
    Mycelium,

    /// Note
    Note,

    /// Poof
    Poof,

    /// Portal
    Portal,

    /// Rain
    Rain,

    /// Smoke
    Smoke,

    /// White Smoke
    WhiteSmoke,

    /// Sneeze
    Sneeze,

    /// Spit
    Spit,

    /// Squid Ink
    SquidInk,

    /// Sweep Attack
    SweepAttack,

    /// Totem Of Undying
    TotemOfUndying,

    /// Under Water
    UnderWater,

    /// Splash
    Splash,

    /// Witch
    Witch,

    /// Bubble Pop
    BubblePop,

    /// Current Down
    CurrentDown,

    /// Bubble Column Up
    BubbleColumnUp,

    /// Nautilus
    Nautilus,

    /// Dolphin
    Dolphin,

    /// Campfire Cosy Smoke
    CampfireCosySmoke,

    /// Campfire Signal Smoke
    CampfireSignalSmoke,

    /// Dripping Honey
    DrippingHoney,

    /// Falling Honey
    FallingHoney,

    /// Landing Honey
    LandingHoney,

    /// Falling Nectar
    FallingNectar,

    /// Falling Spore Blossom
    FallingSporeBlossom,

    /// Ash
    Ash,

    /// Crimson Spore
    CrimsonSpore,

    /// Warped Spore
    WarpedSpore,

    /// Spore Blossom Air
    SporeBlossomAir,

    /// Dripping Obsidian Tear
    DrippingObsidianTear,

    /// Falling Obsidian Tear
    FallingObsidianTear,

    /// Landing Obsidian Tear
    LandingObsidianTear,

    /// Reverse Portal
    ReversePortal,

    /// White Ash
    WhiteAsh,

    /// Small Flame
    SmallFlame,

    /// Snowflake
    Snowflake,

    /// Dripping Dripstone Lava
    DrippingDripstoneLava,

    /// Falling Dripstone Lava
    FallingDripstoneLava,

    /// Dripping Dripstone Water
    DrippingDripstoneWater,

    /// Falling Dripstone Water
    FallingDripstoneWater,

    /// Glow Squid Ink
    GlowSquidInk,

    /// Glow
    Glow,

    /// Wax On
    WaxOn,

    /// Wax Off
    WaxOff,

    /// Electric Spark
    ElectricSpark,

    /// Scrape
    Scrape,

    /// Shriek
    Shriek {
        /// The time in ticks before the particle is displayed.
        delay : u32
    },

    /// Egg Crack
    EggCrack,

    /// Dust Plume
    DustPlume,

    /// Trial Spawner Detection
    TrialSpawnerDetection,

    /// Trial Spawner Detection Ominous
    TrialSpawnerDetectionOminous,

    /// Vault Connection
    VaultConnection,

    /// Dust Pillar
    DustPillar {
        /// The block state.
        state : BlockState
    },

    /// Ominous Spawning
    OminousSpawning,

    /// Raid Omen
    RaidOmen,

    /// Trial Omen
    TrialOmen,

    /// Block Crumble
    BlockCrumble {
        /// The block state.
        state : BlockState
    },

    /// Firefly
    Firefly

}


impl Particle<'_> {

    /// Convert the inner parts of this `Particle` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `Particle<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> Particle<'static> { match (self) {
        Particle::AngryVillager => todo!(),
        Self::Block { state } => Particle::Block { state },
        Self::BlockMarker { state } => Particle::BlockMarker { state },
        Self::Bubble => Particle::Bubble,
        Self::Cloud => Particle::Cloud,
        Self::Crit => Particle::Crit,
        Self::DamageIndicator => Particle::DamageIndicator,
        Self::DragonBreath => Particle::DragonBreath,
        Self::DrippingLava => Particle::DrippingLava,
        Self::FallingLava => Particle::FallingLava,
        Self::LandingLava => Particle::LandingLava,
        Self::DrippingWater => Particle::DrippingWater,
        Self::FallingWater => Particle::FallingWater,
        Self::Dust { colour, scale } => Particle::Dust { colour, scale },
        Self::DustColourTransition { from_colour, to_colour, scale } => Particle::DustColourTransition { from_colour, to_colour, scale },
        Self::Effect => Particle::Effect,
        Self::ElderGuardian => Particle::ElderGuardian,
        Self::EnchantedHit => Particle::EnchantedHit,
        Self::Enchant => Particle::Enchant,
        Self::EndRod => Particle::EndRod,
        Self::EntityEffect { colour } => Particle::EntityEffect { colour },
        Self::ExplosionEmitter => Particle::ExplosionEmitter,
        Self::Explosion => Particle::Explosion,
        Self::Gust => Particle::Gust,
        Self::SmallGust => Particle::SmallGust,
        Self::GustEmitterLarge => Particle::GustEmitterLarge,
        Self::GustEmitterSmall => Particle::GustEmitterSmall,
        Self::SonicBoom => Particle::SonicBoom,
        Self::FallingDust { state } => Particle::FallingDust { state },
        Self::Firework => Particle::Firework,
        Self::Fishing => Particle::Fishing,
        Self::Flame => Particle::Flame,
        Self::Infested => Particle::Infested,
        Self::CherryLeaves => Particle::CherryLeaves,
        Self::PaleOakLeaves => Particle::PaleOakLeaves,
        Self::TintedLeaves { colour } => Particle::TintedLeaves { colour },
        Self::SculkSoul => Particle::SculkSoul,
        Self::SculkCharge { roll } => Particle::SculkCharge { roll },
        Self::SculkChargePop => Particle::SculkChargePop,
        Self::SoulFireFlame => Particle::SoulFireFlame,
        Self::Soul => Particle::Soul,
        Self::Flash => Particle::Flash,
        Self::HappyVillager => Particle::HappyVillager,
        Self::Composter => Particle::Composter,
        Self::Heart => Particle::Heart,
        Self::InstantEffect => Particle::InstantEffect,
        Self::Item { item } => Particle::Item { item : item.into_static_owned() },
        Self::Vibration(todo) => Particle::Vibration(todo),
        Self::Trail { x, y, z, colour, duration } => Particle::Trail { x, y, z, colour, duration },
        Self::ItemSlime => Particle::ItemSlime,
        Self::ItemCobweb => Particle::ItemCobweb,
        Self::ItemSnowball => Particle::ItemSnowball,
        Self::LargeSmoke => Particle::LargeSmoke,
        Self::Lava => Particle::Lava,
        Self::Mycelium => Particle::Mycelium,
        Self::Note => Particle::Note,
        Self::Poof => Particle::Poof,
        Self::Portal => Particle::Portal,
        Self::Rain => Particle::Rain,
        Self::Smoke => Particle::Smoke,
        Self::WhiteSmoke => Particle::WhiteSmoke,
        Self::Sneeze => Particle::Sneeze,
        Self::Spit => Particle::Spit,
        Self::SquidInk => Particle::SquidInk,
        Self::SweepAttack => Particle::SweepAttack,
        Self::TotemOfUndying => Particle::TotemOfUndying,
        Self::UnderWater => Particle::UnderWater,
        Self::Splash => Particle::Splash,
        Self::Witch => Particle::Witch,
        Self::BubblePop => Particle::BubblePop,
        Self::CurrentDown => Particle::CurrentDown,
        Self::BubbleColumnUp => Particle::BubbleColumnUp,
        Self::Nautilus => Particle::Nautilus,
        Self::Dolphin => Particle::Dolphin,
        Self::CampfireCosySmoke => Particle::CampfireCosySmoke,
        Self::CampfireSignalSmoke => Particle::CampfireSignalSmoke,
        Self::DrippingHoney => Particle::DrippingHoney,
        Self::FallingHoney => Particle::FallingHoney,
        Self::LandingHoney => Particle::LandingHoney,
        Self::FallingNectar => Particle::FallingNectar,
        Self::FallingSporeBlossom => Particle::FallingSporeBlossom,
        Self::Ash => Particle::Ash,
        Self::CrimsonSpore => Particle::CrimsonSpore,
        Self::WarpedSpore => Particle::WarpedSpore,
        Self::SporeBlossomAir => Particle::SporeBlossomAir,
        Self::DrippingObsidianTear => Particle::DrippingObsidianTear,
        Self::FallingObsidianTear => Particle::FallingObsidianTear,
        Self::LandingObsidianTear => Particle::LandingObsidianTear,
        Self::ReversePortal => Particle::ReversePortal,
        Self::WhiteAsh => Particle::WhiteAsh,
        Self::SmallFlame => Particle::SmallFlame,
        Self::Snowflake => Particle::Snowflake,
        Self::DrippingDripstoneLava => Particle::DrippingDripstoneLava,
        Self::FallingDripstoneLava => Particle::FallingDripstoneLava,
        Self::DrippingDripstoneWater => Particle::DrippingDripstoneWater,
        Self::FallingDripstoneWater => Particle::FallingDripstoneWater,
        Self::GlowSquidInk => Particle::GlowSquidInk,
        Self::Glow => Particle::Glow,
        Self::WaxOn => Particle::WaxOn,
        Self::WaxOff => Particle::WaxOff,
        Self::ElectricSpark => Particle::ElectricSpark,
        Self::Scrape => Particle::Scrape,
        Self::Shriek { delay } => Particle::Shriek { delay },
        Self::EggCrack => Particle::EggCrack,
        Self::DustPlume => Particle::DustPlume,
        Self::TrialSpawnerDetection => Particle::TrialSpawnerDetection,
        Self::TrialSpawnerDetectionOminous => Particle::TrialSpawnerDetectionOminous,
        Self::VaultConnection => Particle::VaultConnection,
        Self::DustPillar { state } => Particle::DustPillar { state },
        Self::OminousSpawning => Particle::OminousSpawning,
        Self::RaidOmen => Particle::RaidOmen,
        Self::TrialOmen => Particle::TrialOmen,
        Self::BlockCrumble { state } => Particle::BlockCrumble { state },
        Self::Firefly => Particle::Firefly,
    } }

    /// Convert the inner parts of this `Particle` to their owned counterparts.
    ///  Returns the newly created `Particle<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> Particle<'static> { match (self) {
        Particle::AngryVillager => todo!(),
        Self::Block { state } => Particle::Block { state : *state },
        Self::BlockMarker { state } => Particle::BlockMarker { state : *state },
        Self::Bubble => Particle::Bubble,
        Self::Cloud => Particle::Cloud,
        Self::Crit => Particle::Crit,
        Self::DamageIndicator => Particle::DamageIndicator,
        Self::DragonBreath => Particle::DragonBreath,
        Self::DrippingLava => Particle::DrippingLava,
        Self::FallingLava => Particle::FallingLava,
        Self::LandingLava => Particle::LandingLava,
        Self::DrippingWater => Particle::DrippingWater,
        Self::FallingWater => Particle::FallingWater,
        Self::Dust { colour, scale } => Particle::Dust { colour : *colour, scale : *scale },
        Self::DustColourTransition { from_colour, to_colour, scale } => Particle::DustColourTransition { from_colour : *from_colour, to_colour : *to_colour, scale : *scale },
        Self::Effect => Particle::Effect,
        Self::ElderGuardian => Particle::ElderGuardian,
        Self::EnchantedHit => Particle::EnchantedHit,
        Self::Enchant => Particle::Enchant,
        Self::EndRod => Particle::EndRod,
        Self::EntityEffect { colour } => Particle::EntityEffect { colour : *colour },
        Self::ExplosionEmitter => Particle::ExplosionEmitter,
        Self::Explosion => Particle::Explosion,
        Self::Gust => Particle::Gust,
        Self::SmallGust => Particle::SmallGust,
        Self::GustEmitterLarge => Particle::GustEmitterLarge,
        Self::GustEmitterSmall => Particle::GustEmitterSmall,
        Self::SonicBoom => Particle::SonicBoom,
        Self::FallingDust { state } => Particle::FallingDust { state : *state },
        Self::Firework => Particle::Firework,
        Self::Fishing => Particle::Fishing,
        Self::Flame => Particle::Flame,
        Self::Infested => Particle::Infested,
        Self::CherryLeaves => Particle::CherryLeaves,
        Self::PaleOakLeaves => Particle::PaleOakLeaves,
        Self::TintedLeaves { colour } => Particle::TintedLeaves { colour : *colour },
        Self::SculkSoul => Particle::SculkSoul,
        Self::SculkCharge { roll } => Particle::SculkCharge { roll : *roll },
        Self::SculkChargePop => Particle::SculkChargePop,
        Self::SoulFireFlame => Particle::SoulFireFlame,
        Self::Soul => Particle::Soul,
        Self::Flash => Particle::Flash,
        Self::HappyVillager => Particle::HappyVillager,
        Self::Composter => Particle::Composter,
        Self::Heart => Particle::Heart,
        Self::InstantEffect => Particle::InstantEffect,
        Self::Item { item } => Particle::Item { item : item.to_static_owned() },
        Self::Vibration(_) => todo!(),
        Self::Trail { x, y, z, colour, duration } => Particle::Trail { x : *x, y : *y, z : *z, colour : *colour, duration : *duration },
        Self::ItemSlime => Particle::ItemSlime,
        Self::ItemCobweb => Particle::ItemCobweb,
        Self::ItemSnowball => Particle::ItemSnowball,
        Self::LargeSmoke => Particle::LargeSmoke,
        Self::Lava => Particle::Lava,
        Self::Mycelium => Particle::Mycelium,
        Self::Note => Particle::Note,
        Self::Poof => Particle::Poof,
        Self::Portal => Particle::Portal,
        Self::Rain => Particle::Rain,
        Self::Smoke => Particle::Smoke,
        Self::WhiteSmoke => Particle::WhiteSmoke,
        Self::Sneeze => Particle::Sneeze,
        Self::Spit => Particle::Spit,
        Self::SquidInk => Particle::SquidInk,
        Self::SweepAttack => Particle::SweepAttack,
        Self::TotemOfUndying => Particle::TotemOfUndying,
        Self::UnderWater => Particle::UnderWater,
        Self::Splash => Particle::Splash,
        Self::Witch => Particle::Witch,
        Self::BubblePop => Particle::BubblePop,
        Self::CurrentDown => Particle::CurrentDown,
        Self::BubbleColumnUp => Particle::BubbleColumnUp,
        Self::Nautilus => Particle::Nautilus,
        Self::Dolphin => Particle::Dolphin,
        Self::CampfireCosySmoke => Particle::CampfireCosySmoke,
        Self::CampfireSignalSmoke => Particle::CampfireSignalSmoke,
        Self::DrippingHoney => Particle::DrippingHoney,
        Self::FallingHoney => Particle::FallingHoney,
        Self::LandingHoney => Particle::LandingHoney,
        Self::FallingNectar => Particle::FallingNectar,
        Self::FallingSporeBlossom => Particle::FallingSporeBlossom,
        Self::Ash => Particle::Ash,
        Self::CrimsonSpore => Particle::CrimsonSpore,
        Self::WarpedSpore => Particle::WarpedSpore,
        Self::SporeBlossomAir => Particle::SporeBlossomAir,
        Self::DrippingObsidianTear => Particle::DrippingObsidianTear,
        Self::FallingObsidianTear => Particle::FallingObsidianTear,
        Self::LandingObsidianTear => Particle::LandingObsidianTear,
        Self::ReversePortal => Particle::ReversePortal,
        Self::WhiteAsh => Particle::WhiteAsh,
        Self::SmallFlame => Particle::SmallFlame,
        Self::Snowflake => Particle::Snowflake,
        Self::DrippingDripstoneLava => Particle::DrippingDripstoneLava,
        Self::FallingDripstoneLava => Particle::FallingDripstoneLava,
        Self::DrippingDripstoneWater => Particle::DrippingDripstoneWater,
        Self::FallingDripstoneWater => Particle::FallingDripstoneWater,
        Self::GlowSquidInk => Particle::GlowSquidInk,
        Self::Glow => Particle::Glow,
        Self::WaxOn => Particle::WaxOn,
        Self::WaxOff => Particle::WaxOff,
        Self::ElectricSpark => Particle::ElectricSpark,
        Self::Scrape => Particle::Scrape,
        Self::Shriek { delay } => Particle::Shriek { delay : *delay },
        Self::EggCrack => Particle::EggCrack,
        Self::DustPlume => Particle::DustPlume,
        Self::TrialSpawnerDetection => Particle::TrialSpawnerDetection,
        Self::TrialSpawnerDetectionOminous => Particle::TrialSpawnerDetectionOminous,
        Self::VaultConnection => Particle::VaultConnection,
        Self::DustPillar { state } => Particle::DustPillar { state : *state },
        Self::OminousSpawning => Particle::OminousSpawning,
        Self::RaidOmen => Particle::RaidOmen,
        Self::TrialOmen => Particle::TrialOmen,
        Self::BlockCrumble { state } => Particle::BlockCrumble { state : *state },
        Self::Firefly => Particle::Firefly,
    } }

}

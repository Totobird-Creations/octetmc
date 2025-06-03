//! Particles


use crate::value::item_slot::ItemSlot;
use crate::value::rgb::{ Rgb, Argb };
use crate::value::block_state::BlockState;


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

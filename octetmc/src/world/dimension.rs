//! World dimension settings.


use super::biome::Biome;
use octetmc_protocol::packet::config::s2c::registry_data::RegistryEntry;
use octetmc_protocol::value::ident::Ident;
use octetmc_protocol::value::nbt::{ Nbt, NbtCompound, NbtCompoundEntry, NbtElement };
use core::num::NonZeroU8;
use std::borrow::Cow;


/// The dimension type that a world can be.
#[derive(Clone, Debug)]
pub struct Dimension<'l> {

    /// The ID of this dimension.
    pub id              : Ident<'l>,

    /// If `Some(_)` this dimension has its day time locked.
    pub fixed_time      : Option<u64>,

    /// Whether this dimension has skylight.
    pub has_skylight    : bool,

    /// Whether this dimension has a bedrock ceiling.
    ///  Affects weather.
    pub has_ceiling     : bool,

    /// When false, compasses spin randomly.
    pub compass_stable  : bool,

    /// The minimum Y section.
    ///
    /// The minimum world height will be `16 * min_section`
    pub min_section     : i8,

    /// The number of vertical sections in this world.
    ///
    /// The maximum world height will be `(16 * min_section) + (16 * height_sections)`.
    pub height_sections : NonZeroU8,

    /// Affects cloud level, sky type, light map, and ambient light.
    pub effects         : DimensionEffects,

    /// How much light the dimension has.
    ///
    /// `0.0` for `minecraft:overworld` and `minecraft:the_end`.
    ///  `0.1` for `minecraft:the_nether`.
    pub ambient_light   : f32,

    /// Piglins shake if `false`.
    pub piglin_safe     : bool,

    /// Whether the world is a [superflat world](https://minecraft.wiki/w/Superflat).
    ///
    /// Superflat worlds have different void fog, and a horizon at y=0 instead of y=63.
    pub is_superflat    : bool,

    /// Used client side for biome noise.
    ///
    /// Vanilla servers use the first 8 bytes of the SHA-256 hash of the world seed.
    pub hashed_seed     : u64,

    /// The world's sea level.
    ///
    /// Default is 64, unless superflat.
    pub sea_level       : i32,

    /// Biome types which can appear in this world.
    pub biomes          : Cow<'l, [Biome<'l>]>

}


impl Dimension<'_> {

    /// Minimal dimension with as few effects as possible.
    pub const MINIMAL : Dimension<'static> = Dimension {
        id              : Ident::new("octetmc", "empty"),
        fixed_time      : Some(6000),
        has_skylight    : true,
        has_ceiling     : false,
        compass_stable  : true,
        min_section     : 0,
        height_sections : unsafe { NonZeroU8::new_unchecked(16) },
        effects         : DimensionEffects::Overworld,
        ambient_light   : 0.0,
        piglin_safe     : true,
        is_superflat    : true,
        hashed_seed     : 0,
        sea_level       : 64,
        biomes          : Cow::Borrowed(const { &[Biome::MINIMAL] })
    };

    /// Vanilla `minecraft:overworld` world with `minecraft:the_void` biome.
    pub const OVERWORLD_THE_VOID : Dimension<'static> = Dimension {
        id              : Ident::new_vanilla("overworld"),
        fixed_time      : None,
        has_skylight    : true,
        has_ceiling     : false,
        compass_stable  : true,
        min_section     : -4,
        height_sections : unsafe { NonZeroU8::new_unchecked(28) },
        effects         : DimensionEffects::Overworld,
        ambient_light   : 0.0,
        piglin_safe     : false,
        is_superflat    : true,
        hashed_seed     : 0,
        sea_level       : 64,
        biomes          : Cow::Borrowed(const { &[Biome::THE_VOID] })
    };

}


/// Affects cloud level, sky type, light map, and ambient light.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DimensionEffects {

    /// Clouds at 192, normal sky type, normal light map, and normal ambient light.
    Overworld,

    /// **No clouds**, nether sky type, normal light map, and **constant ambient light**.
    Nether,

    /// **No clouds**, end sky type, **forced light map**, and normal ambient light.
    End

}


impl Dimension<'_> {

    /// Convert `self` to a [`RegistryEntry`].
    pub fn to_registry_entry(&self) -> RegistryEntry {
        let height = (self.height_sections.get() as i32) * 16;
        let mut entries = vec![
            NbtCompoundEntry {
                key   : Cow::Borrowed("has_skylight"),
                value : NbtElement::Byte(if (self.has_skylight) { 1 } else { 0 })
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("has_ceiling"),
                value : NbtElement::Byte(if (self.has_ceiling) { 1 } else { 0 })
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("ultrawarm"),
                value : NbtElement::Byte(0)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("natural"),
                value : NbtElement::Byte(if (self.compass_stable) { 1 } else { 0 })
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("coordinate_scale"),
                value : NbtElement::Double(1.0)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("bed_works"),
                value : NbtElement::Byte(1)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("respawn_anchor_works"),
                value : NbtElement::Byte(1)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("min_y"),
                value : NbtElement::Int((self.min_section as i32) * 16)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("height"),
                value : NbtElement::Int(height)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("logical_height"),
                value : NbtElement::Int(height)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("infiniburn"),
                value : NbtElement::String(Cow::Borrowed("#"))
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("effects"),
                value : NbtElement::String(Cow::Borrowed(match (self.effects) {
                    DimensionEffects::Overworld => "minecraft:overworld",
                    DimensionEffects::Nether    => "minecraft:the_nether",
                    DimensionEffects::End       => "minecraft:the_end"
                }))
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("ambient_light"),
                value : NbtElement::Float(self.ambient_light)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("piglin_safe"),
                value : NbtElement::Byte(if (self.piglin_safe) { 1 } else { 0 })
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("has_raids"),
                value : NbtElement::Byte(1)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("monster_spawn_light_level"),
                value : NbtElement::Int(0)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("monster_spawn_block_light_limit"),
                value : NbtElement::Int(0)
            }
        ];
        if let Some(fixed_time) = self.fixed_time {
            entries.push(NbtCompoundEntry {
                key   : Cow::Borrowed("fixed_time"),
                value : NbtElement::Long(fixed_time as i64)
            });
        }
        RegistryEntry {
            id   : self.id.as_ref(),
            data : Some(Nbt::from(NbtCompound { entries : Cow::Owned(entries) })),
        }
    }

}

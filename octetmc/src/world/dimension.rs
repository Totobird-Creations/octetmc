//! World dimension settings.


use super::biome::Biome;
use octetmc_protocol::value::ident::Ident;
use std::borrow::Cow;


/// The dimension type that a world can be.
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

    /// The minimum Y level.
    pub min_y           : i32,

    /// The number of vertical sections in this world.
    ///
    /// The maximum world height will be `min_y + (16 * height_sections)`.
    pub height_sections : u8,

    /// Affects cloud level, sky type, light map, and ambient light.
    ///
    /// - `minecraft:overworld` for clouds at 192, normal sky type, normal light map, and normal ambient light.
    /// - `minecraft:the_nether` for **no clouds**, nether sky type, normal light map, and **constant ambient light**.
    /// - `minecraft:the_end` for **no clouds**, end sky type, **forced light map**, and normal ambient light.
    ///
    pub effects         : Ident<'l>,

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

    /// The world's sea level.
    ///
    /// Default is 64, unless superflat.
    pub sea_level       : i32,

    /// Biome types which can appear in this world.
    pub biomes          : Cow<'l, [Biome<'l>]>

}

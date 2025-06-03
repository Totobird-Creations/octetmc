//! World biome settings.


use octetmc_protocol::value::ident::Ident;
use octetmc_protocol::value::rgb::Rgb;
use octetmc_protocol::value::particle::Particle;


/// Biomes types which can appear in a world.
#[derive(Debug, Clone)]
pub struct Biome<'l> {

    /// The ID of the biome.
    pub id                    : Ident<'l>,

    /// Whether the biome has precipitation.
    pub has_precipitation     : bool,

    /// The temperature factor of the biome.
    ///  Affects foliage and grass color if they are not explicitly set.
    ///
    /// The default values vary between -0.5 and 2.0.
    pub termperature          : f32,

    /// Modifier that affects the resulting temperature.
    pub temperature_modifier  : BiomeTemperatureModifier,

    /// The downfall factor of the biome.
    ///  Affects foliage and grass color if they are not explicitly set.
    ///
    /// The default values vary between 0.0 and 1.0.
    pub downfall              : f32,

    /// The colour of the fog effect when looking past the view distance.
    pub fog_colour            : Rgb,

    /// The tint colour of water blocks.
    pub water_colour          : Rgb,

    /// The colour of the fog effect when looking past the view distance when underwater.
    pub water_fog_colour      : Rgb,

    /// The colour of the sky.
    pub sky_colour            : Rgb,

    /// The tint color of leaves.
    ///
    /// If not specified, the foliage colour is calculated based on biome temperature and downfall.
    pub foliage_colour        : Option<Rgb>,

    /// The tint color of the grass.
    ///
    /// If not specified, the grass colour is calculated based on biome temperature and downfall.
    pub grass_colour          : Option<Rgb>,

    /// Modifier that affects the resulting grass colour.
    pub grass_colour_modifier : BiomeGrassColourModifier,

    /// Ambient visual particles.
    pub particle              : Option<BiomeParticle<'l>>,

    /// Ambient soundtrack that starts playing when entering the biome, and fades out when exiting it.
    pub ambient_sound         : Option<BiomeAmbientSound<'l>>,

    /// Additional ambient sound that plays in moody situations. Moodiness increases when blocks
    ///  around the player are at both sky and block light level zero, and decreases otherwise.
    /// The moodiness calculation happens once per tick, and after reaching a certain value, the
    ///  ambient mood sound is played.
    pub mood_sound            : Option<BiomeMoodSound<'l>>,

    /// Additional ambient sound that has a chance of playing randomly every tick.
    pub additions_sound       : Option<BiomeAdditionsSound<'l>>,

    /// Music properties for the biome.
    pub music                 : Option<BiomeMusic<'l>>

}


/// Modifier that affects the resulting temperature.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BiomeTemperatureModifier {

    /// Static temperature throughout the biome (aside from variations depending on the height).
    None,

    /// Pockets of warm temperature (0.2) randomly distributed throughout the biome.
    ///
    /// This is used by frozen ocean variants in the vanilla game to simulate spots of unfrozen
    ///  water, where it always rains instead of snowing.
    Frozen

}


/// Modifier that affects the resulting grass colour.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BiomeGrassColourModifier {

    /// Static grass colour throughout the biome.
    None,

    /// Darker, and less saturated shade of the colour.
    DarkForest,

    /// Overridden with two fixed colors (`#4C763C` and `#6A7039`),
    ///  randomly distributed throughout the biome.
    Swamp

}


/// Ambient visual particles.
#[derive(Debug, Clone)]
pub struct BiomeParticle<'l> {

    /// The particle to spawn.
    pub particle    : Particle<'l>,

    /// The chance for the particle to be spawned.
    ///
    /// Ambient particles are attempted to be spawned multiple times every tick.
    pub probability : f32

}


/// Ambient soundtrack that starts playing when entering the biome, and fades out when exiting it.
#[derive(Debug, Clone)]
pub struct BiomeAmbientSound<'l> {

    /// The ID of a soundtrack.
    pub sound : Ident<'l>,

    /// The range of the sound. If not specified, the volume is used to calculate the effective range.
    pub range : Option<f32>

}


/// Additional ambient sound that plays in moody situations. Moodiness increases when blocks
///  around the player are at both sky and block light level zero, and decreases otherwise.
/// The moodiness calculation happens once per tick, and after reaching a certain value, the
///  ambient mood sound is played.
#[derive(Debug, Clone)]
pub struct BiomeMoodSound<'l> {

    /// The ID of a soundtrack.
    pub sound               : Ident<'l>,

    /// Defines the rate at which the moodiness increase, and also the minimum time between plays.
    ///
    /// The default value is always 6000.
    pub tick_delay          : u32,

    /// The radius used for the block search around the player during moodiness calculation.
    ///
    /// The default value is always 8.
    pub block_search_extent : u32,

    /// The distance offset from the player when playing the sound.
    ///
    /// The sound plays in the direction of the selected block during moodiness calculation, and
    ///  is magnified by the offset.
    ///
    /// The default value is always 2.0.
    pub offset              : f64

}


/// Additional ambient sound that has a chance of playing randomly every tick.
#[derive(Debug, Clone)]
pub struct BiomeAdditionsSound<'l> {

    /// The ID of a soundtrack.
    pub sound       : Ident<'l>,

    /// The chance of the sound playing during the tick.
    pub tick_chance : f64

}


/// Music properties for the biome.
#[derive(Debug, Clone)]
pub struct BiomeMusic<'l> {

    /// The ID of a soundtrack.
    pub sound                 : Ident<'l>,

    /// The minimum time in ticks since the last music finished for this music to be able to play.
    pub min_delay             : u32,

    /// The maximum time in ticks since the last music finished for this music to be able to play.
    pub max_delay             : u32,

    /// Whether this music can replace the current one.
    pub replace_current_music : bool

}

use crate::value::ident::Ident;
use crate::value::rgb::Rgb;
use crate::value::particle::Particle;
use crate::value::nbt::{ Nbt, NbtCompound, NbtCompoundEntry, NbtElement };
use crate::packet::config::s2c::registry_data::RegistryEntry;
use std::borrow::Cow;


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
    pub temperature           : f32,

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

    /// The tint color of dry foliage.
    ///
    /// If not specified, the dry foliage colour is calculated based on biome temperature and downfall.
    pub dry_foliage_colour    : Option<Rgb>,

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
    pub music                 : Cow<'l, [BiomeMusic<'l>]>,

    /// Volume of music in the biome.
    pub music_volume          : f32

}

include!("../.generated/data/worldgen/biome.rs");


impl Biome<'_> {

    /// Convert the inner parts of this `Biome` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `Biome<'static>`.
    pub fn into_static_owned(self) -> Biome<'static> {
        Biome {
            id                    : self.id.into_static_owned(),
            has_precipitation     : self.has_precipitation,
            temperature           : self.temperature,
            temperature_modifier  : self.temperature_modifier,
            downfall              : self.downfall,
            fog_colour            : self.fog_colour,
            water_colour          : self.water_colour,
            water_fog_colour      : self.water_fog_colour,
            sky_colour            : self.sky_colour,
            foliage_colour        : self.foliage_colour,
            dry_foliage_colour    : self.dry_foliage_colour,
            grass_colour          : self.grass_colour,
            grass_colour_modifier : self.grass_colour_modifier,
            particle              : self.particle.map(|particle| particle.into_static_owned()),
            ambient_sound         : self.ambient_sound.map(|sound| sound.into_static_owned()),
            mood_sound            : self.mood_sound.map(|sound| sound.into_static_owned()),
            additions_sound       : self.additions_sound.map(|sound| sound.into_static_owned()),
            music                 : Cow::Owned(match (self.music) {
                Cow::Borrowed(v) => v.iter().map(|v| v.to_static_owned()).collect::<Vec<_>>(),
                Cow::Owned(v)    => v.into_iter().map(|v| v.to_static_owned()).collect::<Vec<_>>()
            }),
            music_volume          : self.music_volume
        }
    }

    /// Convert the inner parts of this `Biome` to their owned counterparts.
    ///  Returns the newly created `Biome<'static>`.
    pub fn to_static_owned(&self) -> Biome<'static> {
        Biome {
            id                    : self.id.to_static_owned(),
            has_precipitation     : self.has_precipitation,
            temperature           : self.temperature,
            temperature_modifier  : self.temperature_modifier,
            downfall              : self.downfall,
            fog_colour            : self.fog_colour,
            water_colour          : self.water_colour,
            water_fog_colour      : self.water_fog_colour,
            sky_colour            : self.sky_colour,
            foliage_colour        : self.foliage_colour,
            dry_foliage_colour    : self.dry_foliage_colour,
            grass_colour          : self.grass_colour,
            grass_colour_modifier : self.grass_colour_modifier,
            particle              : self.particle.as_ref().map(|particle| particle.to_static_owned()),
            ambient_sound         : self.ambient_sound.as_ref().map(|sound| sound.to_static_owned()),
            mood_sound            : self.mood_sound.as_ref().map(|sound| sound.to_static_owned()),
            additions_sound       : self.additions_sound.as_ref().map(|sound| sound.to_static_owned()),
            music                 : Cow::Owned(self.music.iter().map(|v| v.to_static_owned()).collect::<Vec<_>>()),
            music_volume          : self.music_volume
        }
    }

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


impl BiomeParticle<'_> {

    /// Convert the inner parts of this `BiomeParticle` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `BiomeParticle<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> BiomeParticle<'static> {
        BiomeParticle {
            particle    : self.particle.into_static_owned(),
            probability : self.probability
        }
    }

    /// Convert the inner parts of this `BiomeParticle` to their owned counterparts.
    ///  Returns the newly created `BiomeParticle<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> BiomeParticle<'static> {
        BiomeParticle {
            particle    : self.particle.to_static_owned(),
            probability : self.probability
        }
    }

}


/// Ambient soundtrack that starts playing when entering the biome, and fades out when exiting it.
#[derive(Debug, Clone)]
pub struct BiomeAmbientSound<'l> {

    /// The ID of a soundtrack.
    pub sound : Ident<'l>,

    /// The range of the sound. If not specified, the volume is used to calculate the effective range.
    pub range : Option<f32>

}


impl BiomeAmbientSound<'_> {

    /// Convert the inner parts of this `BiomeAmbientSound` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `BiomeAmbientSound<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> BiomeAmbientSound<'static> {
        BiomeAmbientSound {
            sound : self.sound.into_static_owned(),
            range : self.range
        }
    }

    /// Convert the inner parts of this `BiomeAmbientSound` to their owned counterparts.
    ///  Returns the newly created `BiomeAmbientSound<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> BiomeAmbientSound<'static> {
        BiomeAmbientSound {
            sound : self.sound.to_static_owned(),
            range : self.range
        }
    }

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


impl BiomeMoodSound<'_> {

    /// Convert the inner parts of this `BiomeMoodSound` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `BiomeMoodSound<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> BiomeMoodSound<'static> {
        BiomeMoodSound {
            sound               : self.sound.into_static_owned(),
            tick_delay          : self.tick_delay,
            block_search_extent : self.block_search_extent,
            offset              : self.offset
        }
    }

    /// Convert the inner parts of this `BiomeMoodSound` to their owned counterparts.
    ///  Returns the newly created `BiomeMoodSound<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> BiomeMoodSound<'static> {
        BiomeMoodSound {
            sound               : self.sound.to_static_owned(),
            tick_delay          : self.tick_delay,
            block_search_extent : self.block_search_extent,
            offset              : self.offset
        }
    }

}


/// Additional ambient sound that has a chance of playing randomly every tick.
#[derive(Debug, Clone)]
pub struct BiomeAdditionsSound<'l> {

    /// The ID of a soundtrack.
    pub sound       : Ident<'l>,

    /// The chance of the sound playing during the tick.
    pub tick_chance : f64

}


impl BiomeAdditionsSound<'_> {

    /// Convert the inner parts of this `BiomeAdditionsSound` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `BiomeAdditionsSound<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> BiomeAdditionsSound<'static> {
        BiomeAdditionsSound {
            sound       : self.sound.into_static_owned(),
            tick_chance : self.tick_chance
        }
    }

    /// Convert the inner parts of this `BiomeAdditionsSound` to their owned counterparts.
    ///  Returns the newly created `BiomeAdditionsSound<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> BiomeAdditionsSound<'static> {
        BiomeAdditionsSound {
            sound       : self.sound.to_static_owned(),
            tick_chance : self.tick_chance
        }
    }

}


/// Music properties for the biome.
#[derive(Debug, Clone)]
pub struct BiomeMusic<'l> {

    /// The weight of this entry when choosing a track to play.
    pub weight                : u32,

    /// The ID of a soundtrack.
    pub sound                 : Ident<'l>,

    /// The minimum time in ticks since the last music finished for this music to be able to play.
    pub min_delay             : u32,

    /// The maximum time in ticks since the last music finished for this music to be able to play.
    pub max_delay             : u32,

    /// Whether this music can replace the current one.
    pub replace_current_music : bool

}


impl BiomeMusic<'_> {

    /// Convert the inner parts of this `BiomeMusic` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `BiomeMusic<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> BiomeMusic<'static> {
        BiomeMusic {
            weight                : self.weight,
            sound                 : self.sound.into_static_owned(),
            min_delay             : self.min_delay,
            max_delay             : self.max_delay,
            replace_current_music : self.replace_current_music
        }
    }

    /// Convert the inner parts of this `BiomeMusic` to their owned counterparts.
    ///  Returns the newly created `BiomeMusic<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> BiomeMusic<'static> {
        BiomeMusic {
            weight                : self.weight,
            sound                 : self.sound.to_static_owned(),
            min_delay             : self.min_delay,
            max_delay             : self.max_delay,
            replace_current_music : self.replace_current_music
        }
    }

}


impl Biome<'_> {

    /// Convert `self` to a [`RegistryEntry`].
    pub fn to_registry_entry(&self) -> RegistryEntry {
        let mut effects_entries = vec![
            NbtCompoundEntry {
                key   : Cow::Borrowed("fog_color"),
                value : NbtElement::Int(self.fog_colour.to_u32() as i32)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("water_color"),
                value : NbtElement::Int(self.water_colour.to_u32() as i32)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("water_fog_color"),
                value : NbtElement::Int(self.water_fog_colour.to_u32() as i32)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("sky_color"),
                value : NbtElement::Int(self.sky_colour.to_u32() as i32)
            },
            NbtCompoundEntry {
                key   : Cow::Borrowed("grass_color_modifier"),
                value : NbtElement::String(Cow::Borrowed(match (self.grass_colour_modifier) {
                    BiomeGrassColourModifier::None       => "none",
                    BiomeGrassColourModifier::DarkForest => "dark_forest",
                    BiomeGrassColourModifier::Swamp      => "swamp"
                }))
            }
        ];
        if let Some(colour) = &self.foliage_colour {
            effects_entries.push(NbtCompoundEntry {
                key   : Cow::Borrowed("foliage_color"),
                value : NbtElement::Int(colour.to_u32() as i32)
            })
        }
        if let Some(colour) = &self.dry_foliage_colour {
            effects_entries.push(NbtCompoundEntry {
                key   : Cow::Borrowed("dry_foliage_color"),
                value : NbtElement::Int(colour.to_u32() as i32)
            })
        }
        if let Some(colour) = &self.grass_colour {
            effects_entries.push(NbtCompoundEntry {
                key   : Cow::Borrowed("grass_color"),
                value : NbtElement::Int(colour.to_u32() as i32)
            })
        }
        if let Some(particle) = &self.particle {
            // TODO
        }
        if let Some(sound) = &self.ambient_sound {
            let mut sound_entries = vec![
                NbtCompoundEntry {
                    key   : Cow::Borrowed("sound_id"),
                    value : NbtElement::String(sound.sound.to_str())
                }
            ];
            if let Some(range) = sound.range {
                sound_entries.push(NbtCompoundEntry {
                    key   : Cow::Borrowed("range"),
                    value : NbtElement::Float(range)
                });
            }
            effects_entries.push(NbtCompoundEntry {
                key   : Cow::Borrowed("ambient_sound"),
                value : NbtElement::Compound(NbtCompound { entries : Cow::Owned(sound_entries) })
            })
        }
        if let Some(sound) = &self.mood_sound {
            effects_entries.push(NbtCompoundEntry {
                key   : Cow::Borrowed("mood_sound"),
                value : NbtElement::Compound(NbtCompound { entries : Cow::Owned(vec![
                    NbtCompoundEntry {
                        key   : Cow::Borrowed("sound"),
                        value : NbtElement::String(sound.sound.to_str())
                    },
                    NbtCompoundEntry {
                        key   : Cow::Borrowed("tick_delay"),
                        value : NbtElement::Int(sound.tick_delay as i32)
                    },
                    NbtCompoundEntry {
                        key   : Cow::Borrowed("block_search_extent"),
                        value : NbtElement::Int(sound.block_search_extent as i32)
                    },
                    NbtCompoundEntry {
                        key   : Cow::Borrowed("offset"),
                        value : NbtElement::Double(sound.offset)
                    }
                ]) })
            });
        }
        if let Some(sound) = &self.additions_sound {
            effects_entries.push(NbtCompoundEntry {
                key   : Cow::Borrowed("additions_sound"),
                value : NbtElement::Compound(NbtCompound { entries : Cow::Owned(vec![
                    NbtCompoundEntry {
                        key   : Cow::Borrowed("sound"),
                        value : NbtElement::String(sound.sound.to_str())
                    },
                    NbtCompoundEntry {
                        key   : Cow::Borrowed("tick_chance"),
                        value : NbtElement::Double(sound.tick_chance)
                    }
                ]) })
            });
        }
        if (! self.music.is_empty()) {
            let mut music_entries = vec![];
            for entry in &*self.music {
                music_entries.push(NbtElement::Compound(NbtCompound { entries : Cow::Owned(vec![
                    NbtCompoundEntry {
                        key   : Cow::Borrowed("weight"),
                        value : NbtElement::Int(entry.weight as i32)
                    },
                    NbtCompoundEntry {
                        key   : Cow::Borrowed("data"),
                        value : NbtElement::Compound(NbtCompound { entries : Cow::Owned(vec![
                            NbtCompoundEntry {
                                key   : Cow::Borrowed("sound"),
                                value : NbtElement::String(entry.sound.to_str())
                            },
                            NbtCompoundEntry {
                                key   : Cow::Borrowed("min_delay"),
                                value : NbtElement::Int(entry.min_delay as i32)
                            },
                            NbtCompoundEntry {
                                key   : Cow::Borrowed("max_delay"),
                                value : NbtElement::Int(entry.max_delay as i32)
                            },
                            NbtCompoundEntry {
                                key   : Cow::Borrowed("replace_current_music"),
                                value : NbtElement::Byte(if (entry.replace_current_music) { 1 } else { 0 })
                            }
                        ]) })
                    }
                ]) }));
            }
            effects_entries.push(NbtCompoundEntry {
                key   : Cow::Borrowed("music"),
                value : NbtElement::List(Cow::Owned(music_entries))
            });
        }
        effects_entries.push(NbtCompoundEntry {
            key   : Cow::Borrowed("music_volume"),
            value : NbtElement::Float(self.music_volume)
        });
        RegistryEntry {
            id   : self.id.as_ref(),
            data : Some(Nbt::from(NbtCompound { entries : Cow::Owned(vec![
                NbtCompoundEntry {
                    key   : Cow::Borrowed("has_precipitation"),
                    value : NbtElement::Byte(if (self.has_precipitation) { 1 } else { 0 }),
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("temperature"),
                    value : NbtElement::Float(self.temperature)
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("temperature_modifier"),
                    value : NbtElement::String(Cow::Borrowed(match (self.temperature_modifier) {
                        BiomeTemperatureModifier::None   => "none",
                        BiomeTemperatureModifier::Frozen => "frozen"
                    }))
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("downfall"),
                    value : NbtElement::Float(self.downfall)
                },
                NbtCompoundEntry {
                    key   : Cow::Borrowed("effects"),
                    value : NbtElement::Compound(NbtCompound { entries : Cow::Owned(effects_entries) })
                }
            ]) })),
        }
    }

}

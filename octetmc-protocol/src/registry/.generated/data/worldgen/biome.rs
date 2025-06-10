impl Biome<'_> {

    /// Vanilla `minecraft:deep_ocean` damage type.
    pub const DEEP_OCEAN : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("deep_ocean"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:the_end` damage type.
    pub const THE_END : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("the_end"),
        has_precipitation     : false,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(10518688),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(0),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:old_growth_spruce_taiga` damage type.
    pub const OLD_GROWTH_SPRUCE_TAIGA : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("old_growth_spruce_taiga"),
        has_precipitation     : true,
        temperature           : 0.25,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.8,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8233983),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.old_growth_taiga"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:lukewarm_ocean` damage type.
    pub const LUKEWARM_OCEAN : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("lukewarm_ocean"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4566514),
        water_fog_colour      : Rgb::from_u32(267827),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:mangrove_swamp` damage type.
    pub const MANGROVE_SWAMP : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("mangrove_swamp"),
        has_precipitation     : true,
        temperature           : 0.8,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.9,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(3832426),
        water_fog_colour      : Rgb::from_u32(5077600),
        sky_colour            : Rgb::from_u32(7907327),
        foliage_colour        : Some(Rgb::from_u32(9285927)),
        dry_foliage_colour    : Some(Rgb::from_u32(8082228)),
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::Swamp,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.swamp"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:dark_forest` damage type.
    pub const DARK_FOREST : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("dark_forest"),
        has_precipitation     : true,
        temperature           : 0.7,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.8,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7972607),
        foliage_colour        : None,
        dry_foliage_colour    : Some(Rgb::from_u32(8082228)),
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::DarkForest,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.forest"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:birch_forest` damage type.
    pub const BIRCH_FOREST : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("birch_forest"),
        has_precipitation     : true,
        temperature           : 0.6,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.6,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8037887),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.forest"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:beach` damage type.
    pub const BEACH : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("beach"),
        has_precipitation     : true,
        temperature           : 0.8,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.4,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7907327),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:basalt_deltas` damage type.
    pub const BASALT_DELTAS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("basalt_deltas"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(6840176),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : Some(BiomeParticle {
            particle    : Particle::WhiteAsh,
            probability : 0.118093334
        }),
        ambient_sound         : Some(BiomeAmbientSound {
        sound : Ident::parse_str("minecraft:ambient.basalt_deltas.loop"),
        range : None
    }),
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.basalt_deltas.mood"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : Some(BiomeAdditionsSound {
        sound       : Ident::parse_str("minecraft:ambient.basalt_deltas.additions"),
        tick_chance : 0.0111
    }),
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.nether.basalt_deltas"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:snowy_taiga` damage type.
    pub const SNOWY_TAIGA : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("snowy_taiga"),
        has_precipitation     : true,
        temperature           : -0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.4,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4020182),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8625919),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:stony_peaks` damage type.
    pub const STONY_PEAKS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("stony_peaks"),
        has_precipitation     : true,
        temperature           : 1.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.3,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7776511),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.stony_peaks"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:bamboo_jungle` damage type.
    pub const BAMBOO_JUNGLE : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("bamboo_jungle"),
        has_precipitation     : true,
        temperature           : 0.95,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.9,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7842047),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.bamboo_jungle"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:lush_caves` damage type.
    pub const LUSH_CAVES : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("lush_caves"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.lush_caves"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:snowy_plains` damage type.
    pub const SNOWY_PLAINS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("snowy_plains"),
        has_precipitation     : true,
        temperature           : 0.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8364543),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:sunflower_plains` damage type.
    pub const SUNFLOWER_PLAINS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("sunflower_plains"),
        has_precipitation     : true,
        temperature           : 0.8,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.4,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7907327),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:crimson_forest` damage type.
    pub const CRIMSON_FOREST : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("crimson_forest"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(3343107),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : Some(BiomeParticle {
            particle    : Particle::CrimsonSpore,
            probability : 0.025
        }),
        ambient_sound         : Some(BiomeAmbientSound {
        sound : Ident::parse_str("minecraft:ambient.crimson_forest.loop"),
        range : None
    }),
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.crimson_forest.mood"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : Some(BiomeAdditionsSound {
        sound       : Ident::parse_str("minecraft:ambient.crimson_forest.additions"),
        tick_chance : 0.0111
    }),
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.nether.crimson_forest"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:taiga` damage type.
    pub const TAIGA : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("taiga"),
        has_precipitation     : true,
        temperature           : 0.25,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.8,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8233983),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:dripstone_caves` damage type.
    pub const DRIPSTONE_CAVES : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("dripstone_caves"),
        has_precipitation     : true,
        temperature           : 0.8,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.4,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7907327),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.dripstone_caves"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:flower_forest` damage type.
    pub const FLOWER_FOREST : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("flower_forest"),
        has_precipitation     : true,
        temperature           : 0.7,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.8,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7972607),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.flower_forest"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:savanna` damage type.
    pub const SAVANNA : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("savanna"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:grove` damage type.
    pub const GROVE : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("grove"),
        has_precipitation     : true,
        temperature           : -0.2,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.8,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8495359),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.grove"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:nether_wastes` damage type.
    pub const NETHER_WASTES : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("nether_wastes"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(3344392),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : Some(BiomeAmbientSound {
        sound : Ident::parse_str("minecraft:ambient.nether_wastes.loop"),
        range : None
    }),
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.nether_wastes.mood"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : Some(BiomeAdditionsSound {
        sound       : Ident::parse_str("minecraft:ambient.nether_wastes.additions"),
        tick_chance : 0.0111
    }),
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.nether.nether_wastes"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:deep_dark` damage type.
    pub const DEEP_DARK : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("deep_dark"),
        has_precipitation     : true,
        temperature           : 0.8,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.4,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7907327),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.deep_dark"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:mushroom_fields` damage type.
    pub const MUSHROOM_FIELDS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("mushroom_fields"),
        has_precipitation     : true,
        temperature           : 0.9,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 1.0,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7842047),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:desert` damage type.
    pub const DESERT : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("desert"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.desert"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:plains` damage type.
    pub const PLAINS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("plains"),
        has_precipitation     : true,
        temperature           : 0.8,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.4,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7907327),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:old_growth_pine_taiga` damage type.
    pub const OLD_GROWTH_PINE_TAIGA : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("old_growth_pine_taiga"),
        has_precipitation     : true,
        temperature           : 0.3,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.8,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8168447),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.old_growth_taiga"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:frozen_ocean` damage type.
    pub const FROZEN_OCEAN : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("frozen_ocean"),
        has_precipitation     : true,
        temperature           : 0.0,
        temperature_modifier  : BiomeTemperatureModifier::Frozen,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(3750089),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8364543),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:frozen_peaks` damage type.
    pub const FROZEN_PEAKS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("frozen_peaks"),
        has_precipitation     : true,
        temperature           : -0.7,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.9,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8756735),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.frozen_peaks"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:windswept_gravelly_hills` damage type.
    pub const WINDSWEPT_GRAVELLY_HILLS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("windswept_gravelly_hills"),
        has_precipitation     : true,
        temperature           : 0.2,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.3,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8233727),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:forest` damage type.
    pub const FOREST : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("forest"),
        has_precipitation     : true,
        temperature           : 0.7,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.8,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7972607),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.forest"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:snowy_beach` damage type.
    pub const SNOWY_BEACH : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("snowy_beach"),
        has_precipitation     : true,
        temperature           : 0.05,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.3,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4020182),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8364543),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:jagged_peaks` damage type.
    pub const JAGGED_PEAKS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("jagged_peaks"),
        has_precipitation     : true,
        temperature           : -0.7,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.9,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8756735),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.jagged_peaks"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:ice_spikes` damage type.
    pub const ICE_SPIKES : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("ice_spikes"),
        has_precipitation     : true,
        temperature           : 0.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8364543),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:ocean` damage type.
    pub const OCEAN : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("ocean"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:soul_sand_valley` damage type.
    pub const SOUL_SAND_VALLEY : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("soul_sand_valley"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(1787717),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : Some(BiomeParticle {
            particle    : Particle::Ash,
            probability : 0.00625
        }),
        ambient_sound         : Some(BiomeAmbientSound {
        sound : Ident::parse_str("minecraft:ambient.soul_sand_valley.loop"),
        range : None
    }),
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.soul_sand_valley.mood"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : Some(BiomeAdditionsSound {
        sound       : Ident::parse_str("minecraft:ambient.soul_sand_valley.additions"),
        tick_chance : 0.0111
    }),
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.nether.soul_sand_valley"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:windswept_savanna` damage type.
    pub const WINDSWEPT_SAVANNA : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("windswept_savanna"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:swamp` damage type.
    pub const SWAMP : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("swamp"),
        has_precipitation     : true,
        temperature           : 0.8,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.9,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(6388580),
        water_fog_colour      : Rgb::from_u32(2302743),
        sky_colour            : Rgb::from_u32(7907327),
        foliage_colour        : Some(Rgb::from_u32(6975545)),
        dry_foliage_colour    : Some(Rgb::from_u32(8082228)),
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::Swamp,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.swamp"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:deep_frozen_ocean` damage type.
    pub const DEEP_FROZEN_OCEAN : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("deep_frozen_ocean"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::Frozen,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(3750089),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:pale_garden` damage type.
    pub const PALE_GARDEN : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("pale_garden"),
        has_precipitation     : true,
        temperature           : 0.7,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.8,
        fog_colour            : Rgb::from_u32(8484720),
        water_colour          : Rgb::from_u32(7768221),
        water_fog_colour      : Rgb::from_u32(5597568),
        sky_colour            : Rgb::from_u32(12171705),
        foliage_colour        : Some(Rgb::from_u32(8883574)),
        dry_foliage_colour    : Some(Rgb::from_u32(10528412)),
        grass_colour          : Some(Rgb::from_u32(7832178)),
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 0.0
    };

    /// Vanilla `minecraft:snowy_slopes` damage type.
    pub const SNOWY_SLOPES : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("snowy_slopes"),
        has_precipitation     : true,
        temperature           : -0.3,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.9,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8560639),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.snowy_slopes"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:old_growth_birch_forest` damage type.
    pub const OLD_GROWTH_BIRCH_FOREST : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("old_growth_birch_forest"),
        has_precipitation     : true,
        temperature           : 0.6,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.6,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8037887),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.forest"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:the_void` damage type.
    pub const THE_VOID : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("the_void"),
        has_precipitation     : false,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:eroded_badlands` damage type.
    pub const ERODED_BADLANDS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("eroded_badlands"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : Some(Rgb::from_u32(10387789)),
        dry_foliage_colour    : None,
        grass_colour          : Some(Rgb::from_u32(9470285)),
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.badlands"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:badlands` damage type.
    pub const BADLANDS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("badlands"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : Some(Rgb::from_u32(10387789)),
        dry_foliage_colour    : None,
        grass_colour          : Some(Rgb::from_u32(9470285)),
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.badlands"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:end_midlands` damage type.
    pub const END_MIDLANDS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("end_midlands"),
        has_precipitation     : false,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(10518688),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(0),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:windswept_hills` damage type.
    pub const WINDSWEPT_HILLS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("windswept_hills"),
        has_precipitation     : true,
        temperature           : 0.2,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.3,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8233727),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:warped_forest` damage type.
    pub const WARPED_FOREST : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("warped_forest"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(1705242),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : Some(BiomeParticle {
            particle    : Particle::WarpedSpore,
            probability : 0.01428
        }),
        ambient_sound         : Some(BiomeAmbientSound {
        sound : Ident::parse_str("minecraft:ambient.warped_forest.loop"),
        range : None
    }),
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.warped_forest.mood"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : Some(BiomeAdditionsSound {
        sound       : Ident::parse_str("minecraft:ambient.warped_forest.additions"),
        tick_chance : 0.0111
    }),
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.nether.warped_forest"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:windswept_forest` damage type.
    pub const WINDSWEPT_FOREST : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("windswept_forest"),
        has_precipitation     : true,
        temperature           : 0.2,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.3,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8233727),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:river` damage type.
    pub const RIVER : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("river"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:frozen_river` damage type.
    pub const FROZEN_RIVER : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("frozen_river"),
        has_precipitation     : true,
        temperature           : 0.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(3750089),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8364543),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:sparse_jungle` damage type.
    pub const SPARSE_JUNGLE : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("sparse_jungle"),
        has_precipitation     : true,
        temperature           : 0.95,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.8,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7842047),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.sparse_jungle"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:cold_ocean` damage type.
    pub const COLD_OCEAN : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("cold_ocean"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4020182),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:jungle` damage type.
    pub const JUNGLE : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("jungle"),
        has_precipitation     : true,
        temperature           : 0.95,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.9,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7842047),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.jungle"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:wooded_badlands` damage type.
    pub const WOODED_BADLANDS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("wooded_badlands"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : Some(Rgb::from_u32(10387789)),
        dry_foliage_colour    : None,
        grass_colour          : Some(Rgb::from_u32(9470285)),
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.badlands"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:deep_lukewarm_ocean` damage type.
    pub const DEEP_LUKEWARM_OCEAN : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("deep_lukewarm_ocean"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4566514),
        water_fog_colour      : Rgb::from_u32(267827),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:small_end_islands` damage type.
    pub const SMALL_END_ISLANDS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("small_end_islands"),
        has_precipitation     : false,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(10518688),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(0),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:warm_ocean` damage type.
    pub const WARM_OCEAN : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("warm_ocean"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4445678),
        water_fog_colour      : Rgb::from_u32(270131),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:end_barrens` damage type.
    pub const END_BARRENS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("end_barrens"),
        has_precipitation     : false,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(10518688),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(0),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:savanna_plateau` damage type.
    pub const SAVANNA_PLATEAU : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("savanna_plateau"),
        has_precipitation     : false,
        temperature           : 2.0,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.0,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(7254527),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:end_highlands` damage type.
    pub const END_HIGHLANDS : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("end_highlands"),
        has_precipitation     : false,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(10518688),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(0),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:cherry_grove` damage type.
    pub const CHERRY_GROVE : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("cherry_grove"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.8,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(6141935),
        water_fog_colour      : Rgb::from_u32(6141935),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : Some(Rgb::from_u32(11983713)),
        dry_foliage_colour    : None,
        grass_colour          : Some(Rgb::from_u32(11983713)),
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.cherry_grove"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:stony_shore` damage type.
    pub const STONY_SHORE : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("stony_shore"),
        has_precipitation     : true,
        temperature           : 0.2,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.3,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4159204),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8233727),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:deep_cold_ocean` damage type.
    pub const DEEP_COLD_OCEAN : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("deep_cold_ocean"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.5,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(4020182),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
        ] } ),
        music_volume          : 1.0
    };

    /// Vanilla `minecraft:meadow` damage type.
    pub const MEADOW : Biome<'static> = Biome {
        id                    : Ident::vanilla_str("meadow"),
        has_precipitation     : true,
        temperature           : 0.5,
        temperature_modifier  : BiomeTemperatureModifier::None,
        downfall              : 0.8,
        fog_colour            : Rgb::from_u32(12638463),
        water_colour          : Rgb::from_u32(937679),
        water_fog_colour      : Rgb::from_u32(329011),
        sky_colour            : Rgb::from_u32(8103167),
        foliage_colour        : None,
        dry_foliage_colour    : None,
        grass_colour          : None,
        grass_colour_modifier : BiomeGrassColourModifier::None,
        particle              : None,
        ambient_sound         : None,
        mood_sound            : Some(BiomeMoodSound {
        sound               : Ident::parse_str("minecraft:ambient.cave"),
        tick_delay          : 6000,
        block_search_extent : 8,
        offset              : 2.0
    }),
        additions_sound       : None,
        music                 : Cow::Borrowed(const { &[
            BiomeMusic {
                weight                : 1,
                sound                 : Ident::parse_str("minecraft:music.overworld.meadow"),
                min_delay             : 12000,
                max_delay             : 24000,
                replace_current_music : false
            },
        ] } ),
        music_volume          : 1.0
    };

    /// All vanilla biomes.
    pub const VANILLA_BIOMES : &'static [Biome<'static>] = &[
        Self::DEEP_OCEAN,
        Self::THE_END,
        Self::OLD_GROWTH_SPRUCE_TAIGA,
        Self::LUKEWARM_OCEAN,
        Self::MANGROVE_SWAMP,
        Self::DARK_FOREST,
        Self::BIRCH_FOREST,
        Self::BEACH,
        Self::BASALT_DELTAS,
        Self::SNOWY_TAIGA,
        Self::STONY_PEAKS,
        Self::BAMBOO_JUNGLE,
        Self::LUSH_CAVES,
        Self::SNOWY_PLAINS,
        Self::SUNFLOWER_PLAINS,
        Self::CRIMSON_FOREST,
        Self::TAIGA,
        Self::DRIPSTONE_CAVES,
        Self::FLOWER_FOREST,
        Self::SAVANNA,
        Self::GROVE,
        Self::NETHER_WASTES,
        Self::DEEP_DARK,
        Self::MUSHROOM_FIELDS,
        Self::DESERT,
        Self::PLAINS,
        Self::OLD_GROWTH_PINE_TAIGA,
        Self::FROZEN_OCEAN,
        Self::FROZEN_PEAKS,
        Self::WINDSWEPT_GRAVELLY_HILLS,
        Self::FOREST,
        Self::SNOWY_BEACH,
        Self::JAGGED_PEAKS,
        Self::ICE_SPIKES,
        Self::OCEAN,
        Self::SOUL_SAND_VALLEY,
        Self::WINDSWEPT_SAVANNA,
        Self::SWAMP,
        Self::DEEP_FROZEN_OCEAN,
        Self::PALE_GARDEN,
        Self::SNOWY_SLOPES,
        Self::OLD_GROWTH_BIRCH_FOREST,
        Self::THE_VOID,
        Self::ERODED_BADLANDS,
        Self::BADLANDS,
        Self::END_MIDLANDS,
        Self::WINDSWEPT_HILLS,
        Self::WARPED_FOREST,
        Self::WINDSWEPT_FOREST,
        Self::RIVER,
        Self::FROZEN_RIVER,
        Self::SPARSE_JUNGLE,
        Self::COLD_OCEAN,
        Self::JUNGLE,
        Self::WOODED_BADLANDS,
        Self::DEEP_LUKEWARM_OCEAN,
        Self::SMALL_END_ISLANDS,
        Self::WARM_OCEAN,
        Self::END_BARRENS,
        Self::SAVANNA_PLATEAU,
        Self::END_HIGHLANDS,
        Self::CHERRY_GROVE,
        Self::STONY_SHORE,
        Self::DEEP_COLD_OCEAN,
        Self::MEADOW,
    ];

}

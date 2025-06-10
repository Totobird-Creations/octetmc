use crate::util::one_or_many;
use std::path::Path;
use std::fs::File;
use std::io::Write as _;
use std::collections::HashMap;
use std::borrow::Cow;
use smol::stream::StreamExt;
use smol::fs;
use serde::Deserialize as Deser;
use serde_json::from_reader as read_json;
use convert_case::{ Case, Casing as _ };


pub async fn biome(generated_dir : &Path, target_file : &Path) {
    fs::create_dir_all(target_file.parent().unwrap()).await.unwrap();

    let     entries_dir = generated_dir.join("data/minecraft/worldgen/biome");
    let mut entries     = fs::read_dir(entries_dir).await.unwrap();

    let mut target = File::create(target_file).unwrap();
    write!(target, "impl Biome<'_> {{\n\n").unwrap();

    let mut all_idents = Vec::new();

    while let Some(entry) = entries.try_next().await.unwrap() {
        let entry_path = entry.path();
        let Some(id) = entry_path.file_stem().and_then(|stem| stem.to_str())
            else { panic!("bad entry name {entry_path:?}") };
        println!("Generating biome {:?}", id);

        let Biome {
            carvers : _,
            downfall,
            effects : BiomeEffects {
                fog_colour,
                mood_sound : BiomeMoodSound {
                    block_search_extent : mood_block_search_extent,
                    offset : mood_offset,
                    sound : mood_sound,
                    tick_delay : mood_tick_delay,
                },
                music,
                music_volume : _,
                sky_colour,
                water_colour,
                water_fog_colour,
                foliage_colour,
                dry_foliage_colour : _,
                grass_colour_modifier,
                additions_sound,
                ambient_sound,
                particle,
                grass_colour,
            },
            features : _,
            has_precipitation,
            spawn_costs : _,
            spawners : _,
            temperature,
            creature_spawn_probability : _,
            temperature_modifier,
        } = read_json(File::open(&entry_path).unwrap()).unwrap();
        let foliage_colour  = foliage_colour.map_or(Cow::Borrowed("None"), |v| Cow::Owned(format!("Some(Rgb::from_u32({v}))")));
        let grass_colour    = grass_colour.map_or(Cow::Borrowed("None"), |v| Cow::Owned(format!("Some(Rgb::from_u32({v}))")));
        let particle        = particle.map_or(Cow::Borrowed("None"), |BiomeParticle { options : BiomeParticleOptions { kind }, probability }| {
            let kind = kind.split_at(kind.bytes().position(|b| b == b':').unwrap() + 1).1.to_case(Case::Pascal);
            Cow::Owned(format!(
"Some(BiomeParticle {{
            particle    : Particle::{kind},
            probability : {probability:?}
        }})"))
        } );
        let ambient_sound   = ambient_sound.map_or(Cow::Borrowed("None"), |v| Cow::Owned(format!(
"Some(BiomeAmbientSound {{
        sound : Ident::parse_str({v:?}),
        range : None
    }})"
        )));
        let mood_sound      = format!(
"BiomeMoodSound {{
        sound               : Ident::parse_str({mood_sound:?}),
        tick_delay          : {mood_tick_delay},
        block_search_extent : {mood_block_search_extent},
        offset              : {mood_offset:?}
    }}"
        );
        let additions_sound = additions_sound.map_or(Cow::Borrowed("None"), |BiomeAdditionsSound { sound, tick_chance }| Cow::Owned(format!(
"Some(BiomeAdditionsSound {{
        sound       : Ident::parse_str({sound:?}),
        tick_chance : {tick_chance:?}
    }})"
        )));
        let music           = music.into_iter().map(|BiomeMusic { weight, data : BiomeMusicData { sound, min_delay, max_delay, replace_current_music } }| format!("
            BiomeMusic {{
                weight                : {weight},
                sound                 : Ident::parse_str({sound:?}),
                min_delay             : {min_delay},
                max_delay             : {max_delay},
                replace_current_music : {replace_current_music}
            }},"))
            .collect::<String>();

        let ident = id.to_ascii_uppercase();

        write!(target,
"    /// Vanilla `minecraft:{id}` damage type.
    pub const {ident} : Biome<'static> = Biome {{
        id                    : Ident::vanilla_str(\"{id}\"),
        has_precipitation     : {has_precipitation},
        temperature           : {temperature:?},
        temperature_modifier  : BiomeTemperatureModifier::{temperature_modifier:?},
        downfall              : {downfall:?},
        fog_colour            : Rgb::from_u32({fog_colour}),
        water_colour          : Rgb::from_u32({water_colour}),
        water_fog_colour      : Rgb::from_u32({water_fog_colour}),
        sky_colour            : Rgb::from_u32({sky_colour}),
        foliage_colour        : {foliage_colour},
        grass_colour          : {grass_colour},
        grass_colour_modifier : BiomeGrassColourModifier::{grass_colour_modifier:?},
        particle              : {particle},
        ambient_sound         : {ambient_sound},
        mood_sound            : Some({mood_sound}),
        additions_sound       : {additions_sound},
        music                 : Cow::Borrowed(const {{ &[{music}
        ] }} )
    }};\n\n"
        ).unwrap();

        all_idents.push(ident);
    }

    let len = all_idents.len();
    write!(target,
"    /// All vanilla biomes.
    pub const VANILLA_BIOMES : &'static [Biome<'static>] = &[\n\
"
    ).unwrap();
    for ident in all_idents {
        write!(target, "        Self::{ident},\n").unwrap();
    }
    write!(target, "    ];").unwrap();

    write!(target, "\n\n}}\n").unwrap();
}


#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct Biome {
    #[expect(dead_code)]
    #[serde(deserialize_with = "one_or_many")]
    carvers                    : Vec<String>,
    downfall                   : f32,
    effects                    : BiomeEffects,
    #[expect(dead_code)]
    features                   : Vec<Vec<String>>,
    has_precipitation          : bool,
    #[expect(dead_code)]
    spawn_costs                : HashMap<String, BiomeSpawnCost>,
    #[expect(dead_code)]
    spawners                   : BiomeSpawners,
    temperature                : f32,
    #[expect(dead_code)]
    creature_spawn_probability : Option<f32>,
    #[serde(default)]
    temperature_modifier       : BiomeTemperatureModifier
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct BiomeEffects {
    #[serde(rename = "fog_color")]
    fog_colour            : u32,
    mood_sound            : BiomeMoodSound,
    #[serde(default)]
    music                 : Vec<BiomeMusic>,
    #[expect(dead_code)]
    music_volume          : f32,
    #[serde(rename = "sky_color")]
    sky_colour            : u32,
    #[serde(rename = "water_color")]
    water_colour          : u32,
    #[serde(rename = "water_fog_color")]
    water_fog_colour      : u32,
    #[serde(rename = "foliage_color")]
    foliage_colour        : Option<u32>,
    #[expect(dead_code)]
    #[serde(rename = "dry_foliage_color")]
    dry_foliage_colour    : Option<u32>,
    #[serde(default, rename = "grass_color_modifier")]
    grass_colour_modifier : BiomeGrassColourModifier,
    additions_sound       : Option<BiomeAdditionsSound>,
    ambient_sound         : Option<String>,
    particle              : Option<BiomeParticle>,
    #[serde(rename = "grass_color")]
    grass_colour          : Option<u32>
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct BiomeMoodSound {
    block_search_extent : u8,
    offset              : f32,
    sound               : String,
    tick_delay          : u32
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct BiomeMusic {
    weight : u8,
    data   : BiomeMusicData
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct BiomeMusicData {
    sound                 : String,
    min_delay             : u32,
    max_delay             : u32,
    replace_current_music : bool
}

#[derive(Deser, Debug, Default)]
#[serde(deny_unknown_fields)]
enum BiomeGrassColourModifier {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "dark_forest")]
    DarkForest,
    #[serde(rename = "swamp")]
    Swamp
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct BiomeAdditionsSound {
    sound       : String,
    tick_chance : f32
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct BiomeParticle {
    options     : BiomeParticleOptions,
    probability : f32
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct BiomeParticleOptions {
    #[serde(rename = "type")]
    kind : String
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct BiomeSpawnCost {
    #[expect(dead_code)]
    charge        : f32,
    #[expect(dead_code)]
    energy_budget : f32
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct BiomeSpawners {
    #[expect(dead_code)]
    ambient                    : Vec<BiomeSpawner>,
    #[expect(dead_code)]
    axolotls                   : Vec<BiomeSpawner>,
    #[expect(dead_code)]
    creature                   : Vec<BiomeSpawner>,
    #[expect(dead_code)]
    misc                       : Vec<BiomeSpawner>,
    #[expect(dead_code)]
    monster                    : Vec<BiomeSpawner>,
    #[expect(dead_code)]
    underground_water_creature : Vec<BiomeSpawner>,
    #[expect(dead_code)]
    water_ambient              : Vec<BiomeSpawner>,
    #[expect(dead_code)]
    water_creature             : Vec<BiomeSpawner>
}

#[derive(Debug, Deser)]
#[serde(deny_unknown_fields)]
struct BiomeSpawner {
    #[expect(dead_code)]
    #[serde(rename = "type")]
    kind      : String,
    #[expect(dead_code)]
    #[serde(rename = "maxCount")]
    max_count : u8,
    #[expect(dead_code)]
    #[serde(rename = "minCount")]
    min_count : u8,
    #[expect(dead_code)]
    weight    : u8
}

#[derive(Debug, Deser, Default)]
#[serde(deny_unknown_fields)]
enum BiomeTemperatureModifier {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "frozen")]
    Frozen
}

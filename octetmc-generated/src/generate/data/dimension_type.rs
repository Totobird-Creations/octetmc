use std::path::Path;
use std::fs::File;
use std::io::Write as _;
use smol::stream::StreamExt;
use smol::fs;
use serde::Deserialize as Deser;
use serde_json::from_reader as read_json;


pub async fn dimension_type(generated_dir : &Path, target_file : &Path) {
    fs::create_dir_all(target_file.parent().unwrap()).await.unwrap();

    let     entries_dir = generated_dir.join("data/minecraft/dimension_type");
    let mut entries     = fs::read_dir(entries_dir).await.unwrap();

    let mut target = File::create(target_file).unwrap();
    write!(target, "impl DimensionType<'_> {{\n\n").unwrap();

    let mut all_idents = Vec::new();

    while let Some(entry) = entries.try_next().await.unwrap() {
        let entry_path = entry.path();
        let Some(id) = entry_path.file_stem().and_then(|stem| stem.to_str())
            else { panic!("bad entry name: {entry_path:?}") };
        println!("Generating dimension type {:?}", id);

        let DimensionType {
            ambient_light,
            bed_works : _,
            coordinate_scale : _,
            effects,
            fixed_time,
            has_ceiling,
            has_raids : _,
            has_skylight,
            height,
            infiniburn : _,
            logical_height : _,
            min_y,
            monster_spawn_block_light_limit : _,
            monster_spawn_light_level : _,
            natural,
            piglin_safe,
            respawn_anchor_works : _,
            ultrawarm : _
        } = read_json(File::open(&entry_path).unwrap()).unwrap();
        assert_eq!(min_y.rem_euclid(16), 0);
        let min_section = min_y / 16;
        assert_eq!(height.rem_euclid(16), 0);
        let height_sections = height / 16;
        assert!(height_sections > 0);

        let ident = id.to_ascii_uppercase();

        write!(target,
"    /// Vanilla `minecraft:{id}` dimension type.
    pub const {ident} : DimensionType<'static> = DimensionType {{
        id              : Ident::vanilla_str({id:?}),
        fixed_time      : {fixed_time:?},
        has_skylight    : {has_skylight},
        has_ceiling     : {has_ceiling},
        compass_stable  : {natural},
        min_section     : {min_section},
        height_sections : unsafe {{ NonZeroU8::new_unchecked({height_sections}) }},
        effects         : DimensionEffects::{effects:?},
        ambient_light   : {ambient_light:?},
        piglin_safe     : {piglin_safe}
    }};\n\n"
        ).unwrap();

        all_idents.push(ident);
    }

    write!(target,
"    /// All vanilla dimension types.
    pub const VANILLA_DIMENSION_TYPES : &'static [DimensionType<'static>] = &[\n\
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
struct DimensionType {
    ambient_light                   : f32,
    #[expect(dead_code)]
    bed_works                       : bool,
    #[expect(dead_code)]
    coordinate_scale                : f32,
    effects                         : DimensionEffects,
    fixed_time                      : Option<u32>,
    has_ceiling                     : bool,
    #[expect(dead_code)]
    has_raids                       : bool,
    has_skylight                    : bool,
    height                          : u16,
    #[expect(dead_code)]
    infiniburn                      : String,
    #[expect(dead_code)]
    logical_height                  : u16,
    min_y                           : i16,
    #[expect(dead_code)]
    monster_spawn_block_light_limit : u8,
    #[expect(dead_code)]
    monster_spawn_light_level       : MonsterSpawnLightLevel,
    natural                         : bool,
    piglin_safe                     : bool,
    #[expect(dead_code)]
    respawn_anchor_works            : bool,
    #[expect(dead_code)]
    ultrawarm                       : bool
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
enum DimensionEffects {
    #[serde(rename = "minecraft:overworld")]
    Overworld,
    #[serde(rename = "minecraft:the_nether")]
    Nether,
    #[serde(rename = "minecraft:the_end")]
    End
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields, untagged)]
enum MonsterSpawnLightLevel {
    #[expect(dead_code)]
    Direct(u8),
    #[expect(dead_code)]
    Typed(TypedMonsterSpawnLightLevel)
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields, tag = "type")]
enum TypedMonsterSpawnLightLevel {
    #[serde(rename = "minecraft:uniform")]
    Uniform {
        #[expect(dead_code)]
        max_inclusive : u8,
        #[expect(dead_code)]
        min_inclusive : u8
    }
}

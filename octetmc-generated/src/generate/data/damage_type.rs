use std::path::Path;
use std::fs::File;
use std::io::Write as _;
use smol::stream::StreamExt;
use smol::fs;
use serde::Deserialize as Deser;
use serde_json::from_reader as read_json;


pub async fn damage_type(generated_dir : &Path, target_file : &Path) {
    fs::create_dir_all(target_file.parent().unwrap()).await.unwrap();

    let     entries_dir = generated_dir.join("data/minecraft/damage_type");
    let mut entries     = fs::read_dir(entries_dir).await.unwrap();

    let mut target = File::create(target_file).unwrap();
    write!(target, "impl DamageType<'_> {{\n\n").unwrap();

    let mut all_idents = Vec::new();

    while let Some(entry) = entries.try_next().await.unwrap() {
        let entry_path = entry.path();
        let Some(id) = entry_path.file_stem().and_then(|stem| stem.to_str())
            else { panic!("bad entry name: {entry_path:?}") };
        println!("Generating damage type {:?}", id);

        let DamageType {
            message_id,
            exhaustion : _,
            scaling : _,
            effects,
            message_type,
        } = read_json(File::open(&entry_path).unwrap()).unwrap();

        let ident = id.to_ascii_uppercase();

        write!(target,
"    /// Vanilla `minecraft:{id}` damage type.
    pub const {ident} : DamageType<'static> = DamageType {{
        id           : Ident::vanilla_str({id:?}),
        message_id   : Cow::Borrowed({message_id:?}),
        effects      : DamageEffects::{effects:?},
        message_type : DeathMessageType::{message_type:?}
    }};\n\n"
        ).unwrap();

        all_idents.push(ident);
    }

    let len = all_idents.len();
    write!(target,
"    /// All vanilla damage types.
    pub const VANILLA_DAMAGE_TYPES : &'static [DamageType<'static>] = &[\n\
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
struct DamageType {
    message_id   : String,
    #[expect(dead_code)]
    exhaustion   : f32,
    #[expect(dead_code)]
    scaling      : DamageScaling,
    #[serde(default)]
    effects      : DamageEffects,
    #[serde(default, rename = "death_message_type")]
    message_type : DeathMessageType
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
enum DamageScaling {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "when_caused_by_living_non_player")]
    WhenCausedByLivingNonPlayer
}

#[derive(Deser, Debug, Default)]
#[serde(deny_unknown_fields)]
enum DamageEffects {
    #[default]
    #[serde(rename = "hurt")]
    Hurt,
    #[serde(rename = "thorns")]
    Thorns,
    #[serde(rename = "drowning")]
    Drowning,
    #[serde(rename = "burning")]
    Burning,
    #[serde(rename = "poking")]
    Poking,
    #[serde(rename = "freezing")]
    Freezing
}

#[derive(Deser, Debug, Default)]
#[serde(deny_unknown_fields)]
enum DeathMessageType {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "fall_variants")]
    FallVariants,
    #[serde(rename = "intentional_game_design")]
    IntentionalGameDesign
}

use std::path::Path;
use std::fs::File;
use std::io::Write as _;
use smol::stream::StreamExt;
use smol::fs;
use serde::Deserialize as Deser;
use serde::de::IgnoredAny;
use serde_json::from_reader as read_json;


pub async fn wolf_sound_variant(generated_dir : &Path, target_file : &Path) {
    fs::create_dir_all(target_file.parent().unwrap()).await.unwrap();

    let     entries_dir = generated_dir.join("data/minecraft/wolf_sound_variant");
    let mut entries     = fs::read_dir(entries_dir).await.unwrap();

    let mut target = File::create(target_file).unwrap();
    write!(target, "impl WolfSoundVariant<'_> {{\n\n").unwrap();

    let mut all_idents = Vec::new();

    while let Some(entry) = entries.try_next().await.unwrap() {
        let entry_path = entry.path();
        let Some(id) = entry_path.file_stem().and_then(|stem| stem.to_str())
            else { panic!("bad entry name: {entry_path:?}") };
        println!("Generating wolf sound variant {:?}", id);

        let WolfSoundVariant {
            ambient_sound,
            death_sound,
            growl_sound,
            hurt_sound,
            pant_sound,
            whine_sound,
        } = read_json(File::open(&entry_path).unwrap()).unwrap();

        let ident = id.to_ascii_uppercase();

        write!(target,
"    /// Vanilla `minecraft:{id}` wolf sound variant.
    pub const {ident} : WolfSoundVariant<'static> = WolfSoundVariant {{
        id             : Ident::vanilla_str({id:?}),
        ambient_sound  : Ident::parse_str({ambient_sound:?}),
        death_sound    : Ident::parse_str({death_sound:?}),
        growl_sound    : Ident::parse_str({growl_sound:?}),
        hurt_sound     : Ident::parse_str({hurt_sound:?}),
        pant_sound     : Ident::parse_str({pant_sound:?}),
        whine_sound    : Ident::parse_str({whine_sound:?})
    }};\n\n"
        ).unwrap();

        all_idents.push(ident);
    }

    write!(target,
"    /// All vanilla wolf sound variants.
    pub const VANILLA_WOLF_SOUND_VARIANTS : &'static [WolfSoundVariant<'static>] = &[\n\
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
struct WolfSoundVariant {
    ambient_sound : String,
    death_sound   : String,
    growl_sound   : String,
    hurt_sound    : String,
    pant_sound    : String,
    whine_sound   : String,
}

use std::path::Path;
use std::fs::File;
use std::io::Write as _;
use smol::stream::StreamExt;
use smol::fs;
use serde::Deserialize as Deser;
use serde::de::IgnoredAny;
use serde_json::from_reader as read_json;


pub async fn wolf_variant(generated_dir : &Path, target_file : &Path) {
    fs::create_dir_all(target_file.parent().unwrap()).await.unwrap();

    let     entries_dir = generated_dir.join("data/minecraft/wolf_variant");
    let mut entries     = fs::read_dir(entries_dir).await.unwrap();

    let mut target = File::create(target_file).unwrap();
    write!(target, "impl WolfVariant<'_> {{\n\n").unwrap();

    let mut all_idents = Vec::new();

    while let Some(entry) = entries.try_next().await.unwrap() {
        let entry_path = entry.path();
        let Some(id) = entry_path.file_stem().and_then(|stem| stem.to_str())
            else { panic!("bad entry name: {entry_path:?}") };
        println!("Generating wolf variant {:?}", id);

        let WolfVariant {
            assets : WolfVariantAssets {
                angry : angry_asset_id,
                tame : tame_asset_id,
                wild : wild_asset_id
            },
            spawn_conditions : _
        } = read_json(File::open(&entry_path).unwrap()).unwrap();

        let ident = id.to_ascii_uppercase();

        write!(target,
"    /// Vanilla `minecraft:{id}` wolf variant.
    pub const {ident} : WolfVariant<'static> = WolfVariant {{
        id             : Ident::vanilla_str({id:?}),
        wild_asset_id  : Ident::parse_str({wild_asset_id:?}),
        tame_asset_id  : Ident::parse_str({tame_asset_id:?}),
        angry_asset_id : Ident::parse_str({angry_asset_id:?})
    }};\n\n"
        ).unwrap();

        all_idents.push(ident);
    }

    write!(target,
"    /// All vanilla wolf variants.
    pub const VANILLA_WOLF_VARIANTS : &'static [WolfVariant<'static>] = &[\n\
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
struct WolfVariant {
    assets           : WolfVariantAssets,
    #[expect(dead_code)]
    #[serde(default)]
    spawn_conditions : IgnoredAny
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct WolfVariantAssets {
    angry : String,
    tame  : String,
    wild  : String
}

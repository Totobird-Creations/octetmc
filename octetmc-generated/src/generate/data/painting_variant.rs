use core::num::NonZeroU8;
use std::path::Path;
use std::fs::File;
use std::io::Write as _;
use smol::stream::StreamExt;
use smol::fs;
use serde::Deserialize as Deser;
use serde::de::IgnoredAny;
use serde_json::from_reader as read_json;


pub async fn painting_variant(generated_dir : &Path, target_file : &Path) {
    fs::create_dir_all(target_file.parent().unwrap()).await.unwrap();

    let     entries_dir = generated_dir.join("data/minecraft/painting_variant");
    let mut entries     = fs::read_dir(entries_dir).await.unwrap();

    let mut target = File::create(target_file).unwrap();
    write!(target, "impl PaintingVariant<'_> {{\n\n").unwrap();

    let mut all_idents = Vec::new();

    while let Some(entry) = entries.try_next().await.unwrap() {
        let entry_path = entry.path();
        let Some(id) = entry_path.file_stem().and_then(|stem| stem.to_str())
            else { panic!("bad entry name: {entry_path:?}") };
        println!("Generating painting variant {:?}", id);

        let PaintingVariant {
            asset_id,
            author : _,
            height,
            title : _,
            width,
        } = read_json(File::open(&entry_path).unwrap()).unwrap();
        let width  = width.get();
        let height = height.get();

        let ident = id.to_ascii_uppercase();

        write!(target,
"    /// Vanilla `minecraft:{id}` painting variant.
    pub const {ident} : PaintingVariant<'static> = PaintingVariant {{
        id       : ident![{id}],
        asset_id : ident![{asset_id}],
        width    : unsafe {{ NonZeroU8::new_unchecked({width}) }},
        height   : unsafe {{ NonZeroU8::new_unchecked({height}) }}
    }};\n\n"
        ).unwrap();

        all_idents.push(ident);
    }

    write!(target,
"    /// All vanilla painting variants.
    pub const VANILLA_PAINTING_VARIANTS : &'static [PaintingVariant<'static>] = &[\n\
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
struct PaintingVariant {
    asset_id : String,
    #[expect(dead_code)]
    #[serde(default)]
    author   : IgnoredAny,
    height   : NonZeroU8,
    #[expect(dead_code)]
    title    : IgnoredAny,
    width    : NonZeroU8
}

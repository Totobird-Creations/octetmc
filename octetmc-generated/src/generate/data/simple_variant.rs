use std::path::Path;
use std::fs::File;
use std::io::Write as _;
use std::borrow::Cow;
use smol::stream::StreamExt;
use smol::fs;
use serde::Deserialize as Deser;
use serde::de::IgnoredAny;
use serde_json::from_reader as read_json;


pub async fn simple_variant(generated_dir : &Path, target_file : &Path, ty : &str, cst : &str, id : &str, name : &str) {
    fs::create_dir_all(target_file.parent().unwrap()).await.unwrap();

    let     entries_dir = generated_dir.join("data/minecraft").join(id);
    let mut entries     = fs::read_dir(entries_dir).await.unwrap();

    let mut target = File::create(target_file).unwrap();
    write!(target, "impl {ty}<'_> {{\n\n").unwrap();

    let mut all_idents = Vec::new();

    while let Some(entry) = entries.try_next().await.unwrap() {
        let entry_path = entry.path();
        let Some(id) = entry_path.file_stem().and_then(|stem| stem.to_str())
            else { panic!("bad entry name: {entry_path:?}") };
        println!("Generating {name} {:?}", id);

        let SimpleVariant {
            asset_id,
            model,
            spawn_conditions : _
        } = read_json(File::open(&entry_path).unwrap()).unwrap();
        let model = model.map_or(Cow::Borrowed("None"), |model| Cow::Owned(format!("Some(Cow::Borrowed({model:?}))")));

        let ident = id.to_ascii_uppercase();

        write!(target,
"    /// Vanilla `minecraft:{id}` {name}.
    pub const {ident} : {ty}<'static> = {ty}( SimpleVariant {{
        id       : ident![{id}],
        model    : {model},
        asset_id : ident![{asset_id}]
    }} );\n\n"
        ).unwrap();

        all_idents.push(ident);
    }

    write!(target,
"    /// All vanilla {name}s.
    pub const VANILLA_{cst}S : &'static [{ty}<'static>] = &[\n\
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
struct SimpleVariant {
    asset_id         : String,
    model            : Option<String>,
    #[expect(dead_code)]
    #[serde(default)]
    spawn_conditions : IgnoredAny
}

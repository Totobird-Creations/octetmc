use std::path::Path;
use std::fs::File;
use std::io::Write as _;
use std::collections::HashMap;
use std::borrow::Cow;
use smol::fs;
use serde::Deserialize as Deser;
use serde::de::IgnoredAny;
use serde_json::from_reader as read_json;
use convert_case::{ Case, Casing as _ };


pub async fn blocks(generated_dir : &Path, target_file : &Path) {
    fs::create_dir_all(target_file.parent().unwrap()).await.unwrap();

    let entries_file = generated_dir.join("reports/blocks.json");
    println!("Generating blocks");
    let entries      = read_json::<_, Blocks>(File::open(entries_file).unwrap()).unwrap();

    let mut target = File::create(target_file).unwrap();

    for (id, Block { mut props, states, .. },) in entries {
        let id = id.strip_prefix("minecraft:").unwrap();

        let ident = id.to_case(Case::Pascal);

        write!(target,
"/// `minecraft:{id}` block.
pub mod {id} {{
    /// `minecraft:{id}` block.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct {ident}"
        ).unwrap();

        if (props.is_empty()) {
            write!(target, ";\n").unwrap();
        } else {
            write!(target, " {{\n").unwrap();

            props.retain(|k, values| {
                let key = { if (k == "type") { "kind" } else { k } };
                let is_bool = values.len() == 2 && values.iter().any(|v| v == "true") && values.iter().any(|v| v == "false");
                let ty = { if (is_bool) { Cow::Borrowed("bool") } else { Cow::Owned(key.to_case(Case::Pascal)) } };
                write!(target,
"        /// `{k}` state.
        pub {key} : {ty},
"
                ).unwrap();
                ! is_bool
            } );

            write!(target, "    }}\n").unwrap();

            for (k, values,) in &props {
                let key = { if (k == "type") { "kind" } else { k } };
                let ty = key.to_case(Case::Pascal);
                write!(target,
"    /// `{k}` state.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub enum {ty} {{
"
                ).unwrap();
                for value in values {
                    let mut v = value.to_case(Case::Pascal);
                    if (v.chars().next().is_some_and(|ch| ch >= '0' && ch <= '9')) {
                        v.insert(0, 'N');
                    }
                    write!(target,
"        /// `{value}` variant.
        {v},
"
                    ).unwrap();
                }
                write!(target, "    }}\n").unwrap();
            }

        }

        write!(target,
"    impl super::Block for {ident} {{
    }}
    impl From<{ident}> for super::BlockState {{
        fn from(value : {ident}) -> super::BlockState {{
            todo!();
        }}
    }}
"
        ).unwrap();

        write!(target, "}}\n\n").unwrap();

    }

}


type Blocks = HashMap<String, Block>;

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct Block {
    #[expect(dead_code)]
    #[serde(rename = "definition")]
    def    : IgnoredAny,
    #[serde(default, rename = "properties")]
    props  : HashMap<String, Vec<String>>,
    #[serde(rename = "states")]
    states : Vec<BlockState>
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct BlockState {
    id      : u32,
    #[serde(default, rename = "properties")]
    props   : HashMap<String, String>,
    #[serde(default)]
    default : bool
}

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

    for (id, Block { props, states, .. },) in entries {
        let id = id.strip_prefix("minecraft:").unwrap();

        let ident = id.to_case(Case::Pascal);

        struct PropEntry<'l> {
            k        : &'l str,
            key      : &'l str,
            is_bool  : bool,
            ty       : Cow<'static, str>,
            variants : Vec<(&'l str, String,)>,
            values   : HashMap<&'l str, Cow<'l, str>>
        }
        let mut prop_entries = Vec::new();
        for (k, values,) in &props {
            let      key           = { if (k == "type") { "kind" } else { k } };
            let      is_bool       = values.len() == 2 && values.iter().any(|v| v == "true") && values.iter().any(|v| v == "false");
            let      ty            = if (is_bool) { Cow::Borrowed("bool") } else { Cow::Owned(key.to_case(Case::Pascal)) };
            let mut entry_variants = Vec::new();
            let mut entry_values   = HashMap::new();
            if (is_bool) {
                entry_values.insert("true", Cow::Borrowed("true"));
                entry_values.insert("false", Cow::Borrowed("false"));
            } else {
                for v in values {
                    let mut value = v.to_case(Case::Pascal);
                    if (value.chars().next().is_some_and(|ch| ch >= '0' && ch <= '9')) {
                        value.insert(0, 'N');
                    }
                    entry_values.insert(v.as_str(), Cow::Owned(format!("{ty}::{value}")));
                    entry_variants.push((v.as_str(), value,));
                }
            }
            prop_entries.push(PropEntry {
                k, key, is_bool, ty,
                variants : entry_variants,
                values   : entry_values
            });
        }

        writeln!(target, "/// `minecraft:{id}` block.").unwrap();
        writeln!(target, "pub mod {id} {{").unwrap();
        writeln!(target, "    use super::*;").unwrap();
        writeln!(target).unwrap();
        writeln!(target, "    /// `minecraft:{id}` block.").unwrap();
        writeln!(target, "    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]").unwrap();
        write!(target, "    pub struct {ident}").unwrap();


        if (props.is_empty()) {
            writeln!(target, ";").unwrap();
        } else {
            writeln!(target, " {{").unwrap();

            for PropEntry { k, key, ty, .. } in &prop_entries {
                writeln!(target, "        /// `{k}` state.").unwrap();
                writeln!(target, "        pub {key} : {ty},").unwrap();
            }

            writeln!(target, "    }}").unwrap();

            for PropEntry { k, is_bool, ty, variants, .. } in &prop_entries {
                if (*is_bool) { continue; }
                writeln!(target).unwrap();
                writeln!(target, "    /// `{k}` state.").unwrap();
                writeln!(target, "    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]").unwrap();
                writeln!(target, "    pub enum {ty} {{").unwrap();
                for (v, variant,) in variants {
                    writeln!(target, "        /// `{v}` variant.").unwrap();
                    writeln!(target, "        {variant},").unwrap();
                }
                writeln!(target, "    }}").unwrap();
                writeln!(target, "    impl BlockProperty for {ty} {{ }}").unwrap();
            }

        }


        writeln!(target).unwrap();
        writeln!(target, "    impl {ident} {{").unwrap();
        writeln!(target, "        /// Get this block as a [`BlockState`] ID.").unwrap();
        if (props.is_empty()) {
            writeln!(target, "        #[inline(always)]").unwrap();
        }
        writeln!(target, "        pub const fn to_block_state(&self) -> BlockState {{").unwrap();

        if (props.is_empty()) {
            writeln!(target, "            BlockState::from_raw_id({})", states.first().unwrap().id).unwrap();
        } else {

            write!(target, "            BlockState::from_raw_id(match (( ").unwrap();
            for PropEntry { key, .. } in &prop_entries {
                write!(target, "self.{key}, ").unwrap();
            }
            writeln!(target, ")) {{").unwrap();

            for state in &states {
                write!(target, "                ( ").unwrap();
                for prop_entry in &prop_entries {
                    let value = prop_entry.values.get(state.props.get(prop_entry.k).unwrap().as_str()).unwrap();
                    write!(target, "{value}, ").unwrap();
                }
                writeln!(target, ") => {},", state.id).unwrap();
            }

            writeln!(target, "            }})").unwrap();

        }

        writeln!(target, "        }}").unwrap();
        writeln!(target, "    }}").unwrap();

        writeln!(target).unwrap();
        writeln!(target, "    impl From<{ident}> for BlockState {{").unwrap();
        writeln!(target, "        #[inline(always)]").unwrap();
        writeln!(target, "        fn from(value : {ident}) -> Self {{").unwrap();
        writeln!(target, "            value.to_block_state()").unwrap();
        writeln!(target, "        }}").unwrap();
        writeln!(target, "    }}").unwrap();

        writeln!(target).unwrap();
        writeln!(target, "    impl Default for {ident} {{").unwrap();
        writeln!(target, "        #[inline(always)]").unwrap();
        writeln!(target, "        fn default() -> Self {{").unwrap();
        writeln!(target, "            Self::DEFAULT_STATE").unwrap();
        writeln!(target, "        }}").unwrap();
        writeln!(target, "    }}").unwrap();

        writeln!(target).unwrap();
        writeln!(target, "    impl Block for {ident} {{").unwrap();
        write!(target, "        const DEFAULT_STATE : Self = Self").unwrap();
        if (props.is_empty()) {
            writeln!(target, ";").unwrap();
        } else {
            writeln!(target, " {{").unwrap();
            let state = states.iter().find(|state| state.is_default).unwrap();
            for PropEntry { k, key, values, .. } in &prop_entries {
                let value = values.get(state.props.get(*k).unwrap().as_str()).unwrap();
                writeln!(target, "            {key} : {value},").unwrap();
            }
            writeln!(target, "        }};").unwrap();
        }
        writeln!(target, "    }}").unwrap();

        writeln!(target).unwrap();
        writeln!(target, "}}").unwrap();
        writeln!(target).unwrap();

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
    id         : u32,
    #[serde(default, rename = "properties")]
    props      : HashMap<String, String>,
    #[serde(default, rename = "default")]
    is_default : bool
}

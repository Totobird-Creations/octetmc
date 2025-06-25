use std::path::Path;
use std::fs::File;
use std::io::Write as _;
use std::collections::HashMap;
use smol::fs;
use serde::Deserialize as Deser;
use serde_json::from_reader as read_json;


pub async fn packets(generated_dir : &Path, target_dir : &Path) {
    fs::create_dir_all(target_dir).await.unwrap();

    let entries_file = generated_dir.join("reports/packets.json");
    println!("Generating packets");
    let entries      = read_json::<_, Packets>(File::open(entries_file).unwrap()).unwrap();

    for (state, by_bound,) in entries {
        for (bound, packets,) in by_bound {
            let ident = format!("prefix_check_{state:?}_{bound:?}").to_lowercase();

            let mut target = File::create(target_dir.join(format!("{ident}.rs"))).unwrap();
            writeln!(target, "macro_rules! {ident} {{").unwrap();
            for (id, Packet { prefix }) in packets {
                let id = id.split(":").nth(1).unwrap();
                writeln!(target, "( {id}, 0x{prefix:0>2X} $(,)? ) => {{ {prefix} }};").unwrap();
            }
            writeln!(target, "}}").unwrap();
            writeln!(target, "pub(crate) use {ident};").unwrap();

        }
    }

}


type Packets = HashMap<State, HashMap<Bound, HashMap<String, Packet>>>;

#[derive(Deser)]
#[serde(deny_unknown_fields)]
pub struct Packet {
    #[serde(rename = "protocol_id")]
    prefix : u8
}

#[derive(Deser, PartialEq, Eq, Hash, Debug)]
#[serde(deny_unknown_fields)]
pub enum State {
    #[serde(rename = "handshake")]
    Handshake,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "configuration")]
    Config,
    #[serde(rename = "play")]
    Play
}

#[derive(Deser, PartialEq, Eq, Hash, Debug)]
#[serde(deny_unknown_fields)]
pub enum Bound {
    #[serde(rename = "serverbound")]
    C2S,
    #[serde(rename = "clientbound")]
    S2C
}

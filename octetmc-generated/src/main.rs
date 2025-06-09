use std::path::{ self, PathBuf };
use smol::{ fs, io };


mod get_server_jar_url;
use get_server_jar_url::get_server_jar_url;

mod download_server_jar;
use download_server_jar::download_server_jar;

mod run_datagen;
use run_datagen::run_datagen;

mod generate;


mod util;


fn main() { smol::block_on(async {
    let game_version = "1.21.5";

    println!("Creating working directory...");
    let working_dir = path::absolute(PathBuf::from("octetmc-generated-output")).unwrap();
    match (fs::create_dir(&working_dir).await) {
        Ok(_) => { },
        Err(err) if (err.kind() == io::ErrorKind::AlreadyExists) => { },
        err @ Err(_) => { err.unwrap() }
    }

    println!("Getting server URL...");
    let server_jar_url = get_server_jar_url(game_version).await;

    let server_jar_path = working_dir.join(format!("mc-{game_version}-server.jar"));
    if (! server_jar_path.is_file()) {
        println!("Downloading server jar...");
        let server_jar_pending_path = server_jar_path.with_extension("jar.pending");
        _ = download_server_jar(&server_jar_pending_path, &server_jar_url).await;
        fs::rename(server_jar_pending_path, &server_jar_path).await.unwrap();
    } else {
        println!("Server jar already downloaded.");
    }

    let generated_dir = working_dir.join("generated");
    // println!("Running data generator...");
    // run_datagen(&working_dir, &generated_dir, &server_jar_path).await.unwrap();

    println!("Building Rust code...");
    let target_dir = working_dir.parent().unwrap().parent().unwrap().join("octetmc").join("src").join("generated");

    generate::data::damage_type(&generated_dir, &target_dir.join("data").join("damage_type.rs")).await;
    generate::data::worldgen::biome(&generated_dir, &target_dir.join("data").join("worldgen").join("biome.rs")).await;

}) }

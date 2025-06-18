use std::path::{ self, PathBuf };
use std::fs::File;
use std::io::Write;
use semver::Version;
use smol::{ fs, io };


mod get_server_jar_url;
use get_server_jar_url::get_server_jar_url;

mod download_server_jar;
use download_server_jar::download_server_jar;

mod run_datagen;
use run_datagen::run_datagen;

mod generate;


fn main() { smol::block_on(async {
    let game_version = Version::new(1, 21, 6);

    println!("Creating working directory...");
    let working_dir = path::absolute(PathBuf::from("octetmc-generated-output")).unwrap();
    match (fs::create_dir(&working_dir).await) {
        Ok(_) => { },
        Err(err) if (err.kind() == io::ErrorKind::AlreadyExists) => { },
        err @ Err(_) => { err.unwrap() }
    }

    println!("Getting server URL...");
    let server_jar_url = get_server_jar_url(&game_version.to_string()).await;

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
    println!("Running data generator...");
    run_datagen(&working_dir, &generated_dir, &server_jar_path).await;

    println!("Building Rust code...");
    let root_dir = working_dir.parent().unwrap().parent().unwrap();

    let protocol_target_dir = root_dir.join("octetmc-protocol/src/.generated");
    fs::create_dir_all(&protocol_target_dir).await.unwrap();
    assert!(game_version.pre.is_empty());
    assert!(game_version.build.is_empty());
    let mut f = File::create(protocol_target_dir.join("version.rs")).unwrap();
    writeln!(f, "const _ : () = {{").unwrap();
    writeln!(f, "    const GENERATED : &semver::Version = &semver::Version {{").unwrap();
    writeln!(f, "        major : {},", game_version.major).unwrap();
    writeln!(f, "        minor : {},", game_version.minor).unwrap();
    writeln!(f, "        patch : {},", game_version.patch).unwrap();
    writeln!(f, "        pre   : semver::Prerelease::EMPTY,").unwrap();
    writeln!(f, "        build : semver::BuildMetadata::EMPTY").unwrap();
    writeln!(f, "    }};").unwrap();
    writeln!(f, "    if (LATEST_GAME_VERSION.major != GENERATED.major").unwrap();
    writeln!(f, "        || LATEST_GAME_VERSION.minor != GENERATED.minor").unwrap();
    writeln!(f, "        || LATEST_GAME_VERSION.patch != GENERATED.patch").unwrap();
    writeln!(f, "    ) {{ panic!(\"mismatched generated output version\"); }}").unwrap();
    writeln!(f, "}};").unwrap();

    let packet_target_dir = working_dir.parent().unwrap().parent().unwrap().join("octetmc-protocol/src/packet/.generated");
    generate::packets::packets(&generated_dir, &packet_target_dir).await;

    let registry_target_dir = working_dir.parent().unwrap().parent().unwrap().join("octetmc-protocol/src/registry/.generated");
    generate::data::simple_variant(&generated_dir, &registry_target_dir.join("data").join("cat_variant.rs"), "CatVariant", "CAT_VARIANT", "cat_variant", "cat variant").await;
    generate::data::simple_variant(&generated_dir, &registry_target_dir.join("data").join("chicken_variant.rs"), "ChickenVariant", "CHICKEN_VARIANT", "chicken_variant", "chicken variant").await;
    generate::data::simple_variant(&generated_dir, &registry_target_dir.join("data").join("cow_variant.rs"), "CowVariant", "COW_VARIANT", "cow_variant", "cow variant").await;
    generate::data::damage_type(&generated_dir, &registry_target_dir.join("data").join("damage_type.rs")).await;
    generate::data::dimension_type(&generated_dir, &registry_target_dir.join("data").join("dimension_type.rs")).await;
    generate::data::simple_variant(&generated_dir, &registry_target_dir.join("data").join("frog_variant.rs"), "FrogVariant", "FROG_VARIANT", "frog_variant", "frog variant").await;
    generate::data::painting_variant(&generated_dir, &registry_target_dir.join("data").join("painting_variant.rs")).await;
    generate::data::simple_variant(&generated_dir, &registry_target_dir.join("data").join("pig_variant.rs"), "PigVariant", "PIG_VARIANT", "pig_variant", "pig variant").await;
    generate::data::wolf_sound_variant(&generated_dir, &registry_target_dir.join("data").join("wolf_sound_variant.rs")).await;
    generate::data::wolf_variant(&generated_dir, &registry_target_dir.join("data").join("wolf_variant.rs")).await;
    generate::data::worldgen::biome(&generated_dir, &registry_target_dir.join("data").join("worldgen").join("biome.rs")).await;
    generate::blocks::blocks(&generated_dir, &registry_target_dir.join("blocks.rs")).await;

}) }

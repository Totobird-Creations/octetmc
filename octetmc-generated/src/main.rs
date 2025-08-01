use std::path::{ self, PathBuf };
use std::fs::File;
use std::io::{ self, Write };
use semver::Version;
use smol::fs;
use proguard::{ ProguardMapping, ProguardRecord };
use zip::ZipArchive;
use sha2::{ Sha256, Digest };


mod get_version_info;
use get_version_info::get_version_info;

mod download_server_jar;
use download_server_jar::download_server_jar;

mod run_datagen;
use run_datagen::run_datagen;

mod deobfuscate;

mod generate;


fn main() { smol::block_on(async {
    let game_version        = Version::new(1, 21, 8);
    let game_version_string = game_version.to_string();

    println!("Creating working directory...");
    let working_dir = path::absolute(PathBuf::from("octetmc-generated-output")).unwrap();
    match (fs::create_dir(&working_dir).await) {
        Ok(_) => { },
        Err(err) if (err.kind() == io::ErrorKind::AlreadyExists) => { },
        err @ Err(_) => { err.unwrap() }
    }

    println!("Getting version info...");
    let version_info = get_version_info(&game_version_string).await;

    let server_jar_path            = working_dir.join(format!("mc-{game_version}-server.jar"));
    let should_download_server_jar = ! server_jar_path.is_file();
    if (should_download_server_jar) {
        println!("Downloading server jar...");
        let server_jar_pending_path = server_jar_path.with_extension("jar.pending");
        _ = download_server_jar(&server_jar_pending_path, &version_info.downloads.server.url).await;
        fs::rename(server_jar_pending_path, &server_jar_path).await.unwrap();
    } else {
        println!("Server jar already downloaded.");
    }

    let server_mappings_path            = working_dir.join(format!("mc-{game_version}-mappings.txt"));
    let should_download_server_mappings = should_download_server_jar || ! server_mappings_path.is_file();
    if (should_download_server_mappings) {
        println!("Downloading server deobfuscation mappings...");
        let server_mappings_pending_path = server_mappings_path.with_extension("jar.pending");
        _ = download_server_jar(&server_mappings_pending_path, &version_info.downloads.server_mappings.url).await;
        fs::rename(server_mappings_pending_path, &server_mappings_path).await.unwrap();
    } else {
        println!("Server deobfuscation mappings already downloaded.");
    }
    let server_mappings = fs::read(server_mappings_path).await.unwrap();
    let server_mappings = ProguardMapping::new(&server_mappings);
    let server_mapper   = deobfuscate::ProguardMapper::from(&server_mappings);

    let generated_dir = working_dir.join("generated");
    println!("Skipping data generator.");
    println!("Running data generator...");
    run_datagen(&working_dir, &generated_dir, &server_jar_path).await;

    println!("Building Rust code...");
    let root_dir = working_dir.parent().unwrap().parent().unwrap();

    let protocol_target_dir = root_dir.join("octetmc-protocol/src/.generated");
    fs::create_dir_all(&protocol_target_dir).await.unwrap();

    assert!(game_version.pre.is_empty());
    assert!(game_version.build.is_empty());
    {
        let mut f = File::create(protocol_target_dir.join("version.rs")).unwrap();
        writeln!(f, "const _ : () = {{").unwrap();
        writeln!(f, "    const GENERATED : &semver::Version = &semver::Version {{").unwrap();
        writeln!(f, "        major : {},", game_version.major).unwrap();
        writeln!(f, "        minor : {},", game_version.minor).unwrap();
        writeln!(f, "        patch : {},", game_version.patch).unwrap();
        writeln!(f, "        pre   : semver::Prerelease::EMPTY,").unwrap();
        writeln!(f, "        build : semver::BuildMetadata::EMPTY").unwrap();
        writeln!(f, "    }};").unwrap();
        writeln!(f, "    if (GAME_VERSION.major != GENERATED.major").unwrap();
        writeln!(f, "        || GAME_VERSION.minor != GENERATED.minor").unwrap();
        writeln!(f, "        || GAME_VERSION.patch != GENERATED.patch").unwrap();
        writeln!(f, "    ) {{ panic!(\"mismatched generated output version\"); }}").unwrap();
        writeln!(f, "}};").unwrap();
    }
    {
        let     real_server_jar_path = working_dir.join("versions").join(&game_version_string).join(format!("server-{game_version_string}.jar"));
        let mut server_jar_archive   = ZipArchive::new(File::open(real_server_jar_path).unwrap()).unwrap();

        println!("Generating mapping checks...");
        let mut f = File::create(protocol_target_dir.join("mappings.rs")).unwrap();
        writeln!(f, "macro_rules! mapping_check {{").unwrap();
        for record in server_mappings.iter() {
            if let ProguardRecord::Class { original, obfuscated } = record.unwrap() {
                let mut sha  = Sha256::new();
                deobfuscate::hash_file(&mut sha, &mut server_jar_archive, &server_mapper, obfuscated);
                let     hash = sha.finalize().into_iter().map(|b| format!("{b:0>2x}")).collect::<String>();
                writeln!(f, "    ({original:?} , $hash:tt $(,)? ) => {{").unwrap();
                writeln!(f, "        $crate::mapping_check!{{ @inner , {original:?} , $hash , $ }}").unwrap();
                writeln!(f, "    }};").unwrap();
                writeln!(f, "    (@inner , {original:?} , $hash:tt , $STRING:tt ) => {{").unwrap();
                writeln!(f, "        macro_rules! __mapping_check_hash {{").unwrap();
                writeln!(f, "            ( {hash:?} ) => {{ }};").unwrap();
                writeln!(f, "            ( $STRING hash:tt ) => {{ compile_error!(concat!(\"mismatched hash for class `{original}`. expected `\", $STRING hash, \"` but was actually `{hash}`\")); }};").unwrap();
                writeln!(f, "        }}").unwrap();
                writeln!(f, "        __mapping_check_hash!( $hash );").unwrap();
                writeln!(f, "    }};").unwrap();
            }
        }
        writeln!(f, "    ( $class:tt , $hash:tt $(,)? ) => {{ compile_error!(concat!(\"unknown class `\", $class, \"`. was it renamed?\")); }};").unwrap();
        writeln!(f, "}}").unwrap();
        writeln!(f, "pub(crate) use mapping_check;").unwrap();
    }


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


    let value_target_dir = working_dir.parent().unwrap().parent().unwrap().join("octetmc-protocol/src/value/.generated");

    generate::blocks::blocks(&generated_dir, &value_target_dir.join("blocks.rs")).await;
    generate::registries::registries(&generated_dir, &value_target_dir).await;

}) }

use std::path::Path;
use std::io;
use smol::process::{ Command, Stdio };


pub async fn run_datagen(working_dir : &Path, output_dir : &Path, server_jar : &Path) {
    let status = Command::new("java")
        .arg("-DbundlerMainClass=net.minecraft.data.Main")
        .arg("-jar")
        .arg(server_jar)
        .arg("--output")
        .arg(output_dir)
        .arg("--server")
        .arg("--all")
        .arg("--dev")
        .current_dir(working_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await.unwrap();
    if (! status.success()) {
        panic!("datagen failed");
    }
}

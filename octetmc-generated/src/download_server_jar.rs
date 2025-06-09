use std::path::Path;
use smol::io;
use smol::fs::File;


pub async fn download_server_jar(output_path : &Path, url : &str) -> File {
    let mut file = File::create(output_path)
        .await.unwrap();
    let     resp = surf::get(url)
        .send()
        .await.unwrap();
    io::copy(resp, &mut file)
        .await.unwrap();
    file
}

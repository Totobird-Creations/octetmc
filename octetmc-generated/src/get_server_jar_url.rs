use std::collections::HashMap;
use serde::Deserialize as Deser;


pub async fn get_server_jar_url(version : &str) -> String {
    surf::get("https://raw.githubusercontent.com/liebki/MinecraftServerForkDownloads/refs/heads/main/release_vanilla_downloads.json")
        .send()
        .await.unwrap()
        .body_json::<ReleaseVanillaDownloads>()
        .await.unwrap()
        .versions
        .remove(version).unwrap()
}


#[derive(Deser, Debug)]
struct ReleaseVanillaDownloads {
    #[serde(rename = "server_available")]
    versions : HashMap<String, String>
}

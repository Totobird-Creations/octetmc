use serde::Deserialize as Deser;
use serde::de::IgnoredAny;


pub async fn get_version_info(version : &str) -> VersionInfo {
    let version = surf::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .send()
        .await.unwrap()
        .body_json::<VersionManifest>()
        .await.unwrap()
        .versions
        .into_iter()
        .find(|v| &v.id == version).unwrap();
    surf::get(&version.url)
        .send()
        .await.unwrap()
        .body_json::<VersionInfo>()
        .await.unwrap()
}


#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct VersionManifest {
    #[expect(dead_code)]
    latest   : IgnoredAny,
    versions : Vec<ManifestVersion>
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
struct ManifestVersion {
    id               : String,
    #[expect(dead_code)]
    #[serde(rename = "type")]
    kind             : VersionKind,
    url              : String,
    #[expect(dead_code)]
    time             : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "releaseTime")]
    release_time     : IgnoredAny,
    #[expect(dead_code)]
    sha1             : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "complianceLevel")]
    compliance_level : u8
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
enum VersionKind {
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "old_alpha")]
    OldAlpha
}


#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct VersionInfo {
    #[expect(dead_code)]
    arguments                : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "assetIndex")]
    asset_index              : IgnoredAny,
    #[expect(dead_code)]
    assets                   : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "complianceLevel")]
    compliance_level         : u8,
    pub downloads            : VersionDownloads,
    #[expect(dead_code)]
    id                       : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "javaVersion")]
    java_version             : IgnoredAny,
    #[expect(dead_code)]
    libraries                : IgnoredAny,
    #[expect(dead_code)]
    logging                  : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "mainClass")]
    main_class               : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "minimumLauncherVersion")]
    minimum_launcher_version : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "releaseTime")]
    release_time             : IgnoredAny,
    #[expect(dead_code)]
    time                     : IgnoredAny,
    #[expect(dead_code)]
    #[serde(rename = "type")]
    kind                     : VersionKind
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct VersionDownloads {
    #[expect(dead_code)]
    client              : IgnoredAny,
    #[expect(dead_code)]
    client_mappings     : IgnoredAny,
    pub server          : VersionDownload,
    pub server_mappings : VersionDownload
}

#[derive(Deser, Debug)]
#[serde(deny_unknown_fields)]
pub struct VersionDownload {
    #[expect(dead_code)]
    sha1    : String,
    #[expect(dead_code)]
    size    : usize,
    pub url : String
}

use serde::Deserialize;

pub mod alliance;
pub mod avatar;
pub mod event;
pub mod schedule;
pub mod team;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIIndex {
    pub name: String,
    pub api_version: String,
    pub status: String,
    pub service_manifest_name: String,
    pub service_manifest_version: String,
    pub code_package_name: String,
    pub code_package_version: String,
    pub current_season: u32,
    pub max_season: u32,
}

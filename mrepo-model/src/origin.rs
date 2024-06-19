use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct UpdateJson {
    pub version: String,
    #[cfg(not(feature = "raw"))]
    #[serde(alias = "versionCode")]
    pub version_code: i64,
    #[cfg(feature = "raw")]
    #[serde(rename = "versionCode")]
    pub version_code: i64,
    #[cfg(not(feature = "raw"))]
    #[serde(alias = "zipUrl")]
    pub zip_url: String,
    #[cfg(feature = "raw")]
    #[serde(rename = "zipUrl")]
    pub zip_url: String,
    pub changelog: String,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub version: String,
    #[cfg(not(feature = "raw"))]
    #[serde(alias = "versionCode")]
    pub version_code: i64,
    #[cfg(feature = "raw")]
    #[serde(rename = "versionCode")]
    pub version_code: i64,
    pub author: String,
    pub description: String,
}

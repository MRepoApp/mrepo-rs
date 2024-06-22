use serde::{Deserialize, Serialize};

use crate::origin;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Track {
    pub module: origin::Module,
    pub versions: Vec<Version>,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Version {
    pub timestamp: i64,
    pub version: String,
    pub version_code: i64,
    pub zip_file: String,
    pub changelog: String,
}

impl Version {
    pub fn new(timestamp: i64, version: String, version_code: i64) -> Self {
        Self {
            timestamp,
            version,
            version_code,
            zip_file: format!("{version_code}.zip"),
            changelog: format!("{version_code}.txt"),
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::{config, origin};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Modules {
    pub name: String,
    pub timestamp: i64,
    pub metadata: config::RepositoryMetadata,
    pub modules: Vec<Module>,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub version: String,
    pub version_code: i64,
    pub author: String,
    pub description: String,
    pub metadata: config::ModuleMetadata,
    pub versions: Vec<Version>,
}

impl Module {
    pub fn from(
        origin: origin::Module,
        metadata: config::ModuleMetadata,
        versions: Vec<Version>,
    ) -> Self {
        Self {
            id: origin.id,
            name: origin.name,
            version: origin.version,
            version_code: origin.version_code,
            author: origin.author,
            description: origin.description,
            metadata,
            versions,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Version {
    pub timestamp: i64,
    pub version: String,
    pub version_code: i64,
    pub zip_url: String,
    pub changelog: String,
}

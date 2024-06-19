use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Config {
    pub log: Log,
    pub repository: Repository,
    pub modules: Vec<Module>,
}

impl Config {
    pub fn new(log: Log, repository: Repository, modules: Vec<Module>) -> Self {
        Self {
            log,
            repository,
            modules,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct Log {
    pub disabled: bool,
    pub level: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub output: String,
    pub timestamp: bool,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            disabled: false,
            level: "info".to_owned(),
            output: String::new(),
            timestamp: true,
        }
    }
}

impl Log {
    pub fn new<T: Into<String>>(disabled: bool, level: T, output: T, timestamp: bool) -> Self {
        Self {
            disabled,
            level: level.into(),
            output: output.into(),
            timestamp,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Repository {
    pub name: String,
    #[serde(default)]
    pub metadata: RepositoryMetadata,
    pub setting: RepositorySetting,
}

impl Repository {
    pub fn new<T: Into<String>, M: Into<RepositoryMetadata>>(
        name: T,
        metadata: M,
        setting: RepositorySetting,
    ) -> Self {
        Self {
            name: name.into(),
            metadata: metadata.into(),
            setting,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct RepositoryMetadata {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub homepage: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub donate: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub support: String,
}

impl RepositoryMetadata {
    pub fn new<T: Into<String>>(homepage: T, donate: T, support: T) -> Self {
        Self {
            homepage: homepage.into(),
            donate: donate.into(),
            support: support.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct RepositorySetting {
    pub base_url: String,
    pub keep_size: usize,
}

impl Default for RepositorySetting {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            keep_size: 3,
        }
    }
}

impl RepositorySetting {
    pub fn new<T: Into<String>>(base_url: T, keep_size: usize) -> Self {
        Self {
            base_url: base_url.into(),
            keep_size,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Module {
    pub id: String,
    pub kind: ProviderKind,
    pub provider: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub changelog: String,
    #[serde(default)]
    pub metadata: ModuleMetadata,
    #[serde(default)]
    pub setting: ModuleSetting,
}

impl Module {
    pub fn new<T: Into<String>, M: Into<ModuleMetadata>, S: Into<ModuleSetting>>(
        id: T,
        kind: ProviderKind,
        provider: T,
        changelog: T,
        metadata: M,
        setting: S,
    ) -> Self {
        Self {
            kind,
            id: id.into(),
            provider: provider.into(),
            changelog: changelog.into(),
            metadata: metadata.into(),
            setting: setting.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Copy, Clone)]
pub enum ProviderKind {
    #[serde(rename = "update-json")]
    UpdateJson,
    #[serde(rename = "zip-url")]
    ZipUrl,
    #[cfg(feature = "git")]
    #[serde(rename = "git")]
    Git,
}

#[derive(Deserialize, Serialize, Debug, Default, Eq, PartialEq, Clone)]
#[serde(default)]
pub struct ModuleMetadata {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub license: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub homepage: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub source: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub donate: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub support: String,
}

impl ModuleMetadata {
    pub fn new<T: Into<String>>(license: T, homepage: T, donate: T, support: T, source: T) -> Self {
        Self {
            license: license.into(),
            homepage: homepage.into(),
            donate: donate.into(),
            support: support.into(),
            source: source.into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Eq, PartialEq, Copy, Clone)]
#[serde(default)]
pub struct ModuleSetting {
    pub disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_size: Option<usize>,
}

impl ModuleSetting {
    pub fn new(disabled: bool, keep_size: usize) -> Self {
        Self {
            disabled,
            keep_size: Some(keep_size),
        }
    }
}

macro_rules! impl_from {
    ($t:ty) => {
        impl From<Option<$t>> for $t {
            fn from(value: Option<$t>) -> Self {
                value.unwrap_or_default()
            }
        }
    };
}

impl_from!(Log);
impl_from!(RepositoryMetadata);
impl_from!(ModuleMetadata);
impl_from!(ModuleSetting);

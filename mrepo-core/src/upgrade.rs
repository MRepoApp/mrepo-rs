use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::Utc;
use walkdir::WalkDir;

use mrepo_model::config::{Module, Repository};
use mrepo_model::modules;
use mrepo_model::track::{Track, Version};

use crate::util::{FileUtil, Json};
use crate::{constant, Context};

pub struct Upgrade {
    repository: Repository,
    json_dir: PathBuf,
    modules_dir: PathBuf,
}

impl Upgrade {
    pub fn new<P: AsRef<Path>>(repository: &Repository, json_dir: P, modules_dir: P) -> Self {
        Self {
            repository: repository.to_owned(),
            json_dir: PathBuf::from(json_dir.as_ref()),
            modules_dir: PathBuf::from(modules_dir.as_ref()),
        }
    }

    fn generate_version(&self, id: &String, origin: &Version) -> modules::Version {
        let base_url = &self.repository.setting.base_url;
        let module_path = format!("{}/{}", constant::MODULES_DIR, id);

        modules::Version {
            timestamp: origin.timestamp,
            version: origin.version.to_owned(),
            version_code: origin.version_code,
            zip_url: format!("{base_url}/{module_path}/{}", origin.zip_file),
            changelog: format!("{base_url}/{module_path}/{}", origin.changelog),
        }
    }

    fn generate_module(&self, track: Track, origin: &Module) -> modules::Module {
        let module = &track.module;
        let versions = track
            .versions
            .iter()
            .map(|v| self.generate_version(&module.id, v))
            .collect();

        modules::Module::build(track.module, origin.metadata.to_owned(), versions)
    }

    pub fn generate_modules(&self, origins: &[Arc<Module>]) -> Vec<modules::Module> {
        let mut modules = Vec::new();

        for origin in origins {
            let module_dir = self.modules_dir.join(&origin.id);
            if !module_dir.exists() {
                tracing::warn!(target: "Upgrade::generate_modules", id = %origin.id, "No track found");
                continue;
            }

            let track_json = module_dir.join(constant::TRACK_JSON);
            let track = match Track::from_file(&track_json) {
                Ok(t) => t,
                Err(error) => {
                    tracing::error!(target: "Upgrade::generate_modules", id = %origin.id, ?error);
                    continue;
                }
            };

            modules.push(self.generate_module(track, origin));
        }

        modules
    }

    pub async fn write_modules_to<P: AsRef<Path>>(
        &self,
        modules: &modules::Modules,
        path: P,
        pretty: bool,
    ) -> bool {
        match modules.to_file(path, pretty) {
            Ok(_) => true,
            Err(error) => {
                tracing::error!(target: "Upgrade::write_modules_to", ?error);
                false
            }
        }
    }

    pub async fn remove_unkonwn_path(&self, modules: &[Arc<Module>]) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        let module_ids: Vec<String> = modules.iter().map(|m| m.id.to_owned()).collect();
        let walk_dir = WalkDir::new(&self.modules_dir).min_depth(1).max_depth(1);

        for entry in walk_dir.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                FileUtil::remove(path);
                paths.push(entry.into_path());
                continue;
            }

            let name = entry.file_name().to_str().unwrap_or("").to_owned();
            if !module_ids.contains(&name) {
                FileUtil::remove(path);
                paths.push(entry.into_path());
                continue;
            }
        }

        paths
    }
}

pub struct UpgradeWrapper<'u> {
    modules: &'u Vec<Arc<Module>>,
    original: Arc<Upgrade>,
}

impl<'u> UpgradeWrapper<'u> {
    pub fn build(context: &'u Context) -> Self {
        let upgrade = Upgrade::new(&context.repository, &context.json_dir, &context.modules_dir);

        Self {
            modules: &context.modules,
            original: Arc::new(upgrade),
        }
    }

    pub async fn generate_index_to<P: AsRef<Path>>(&self, path: P, pretty: bool) {
        let modules_new = modules::Modules {
            name: self.repository.name.to_owned(),
            timestamp: Utc::now().timestamp_millis(),
            metadata: self.repository.metadata.to_owned(),
            modules: self.generate_modules(self.modules),
        };

        self.write_modules_to(&modules_new, path, pretty).await;
        self.remove_unkonwn_path(self.modules).await;
    }

    pub async fn generate_index(&self, pretty: bool) {
        let modules_json = self.json_dir.join(constant::MODULES_JSON);
        self.generate_index_to(modules_json, pretty).await
    }
}

impl Deref for UpgradeWrapper<'_> {
    type Target = Upgrade;

    fn deref(&self) -> &Self::Target {
        &self.original
    }
}

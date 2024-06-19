#![allow(unused_assignments)]

use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::Utc;
use tokio::task::JoinHandle;

use mrepo_model::config::{Module, ProviderKind, RepositorySetting};
use mrepo_model::origin;
use mrepo_model::origin::UpdateJson;
use mrepo_model::track::{Track, Version};

use crate::constant;
use crate::util::{FileUtil, Json, LocalModule, Request, StrUtil};
use crate::Context;

pub struct Update {
    setting: RepositorySetting,
    modules_dir: PathBuf,
}

impl Update {
    pub fn new<P: AsRef<Path>>(setting: &RepositorySetting, modules_dir: P) -> Self {
        Self {
            setting: setting.to_owned(),
            modules_dir: PathBuf::from(modules_dir.as_ref()),
        }
    }

    fn check_versions(
        &self,
        module: &Module,
        version: &str,
        version_code: i64,
    ) -> Option<Vec<Version>> {
        let inner = || {
            let version_display = StrUtil::get_version_display(version, version_code);
            tracing::info!(
                target: "Update::check_versions",
                id = %module.id,
                version = %version_display,
                "New version found");
        };

        let module_dir = self.modules_dir.join(&module.id);
        let track_json = module_dir.join(constant::TRACK_JSON);

        let mut track = match Track::from_file(track_json) {
            Ok(t) => t,
            Err(_) => {
                inner();
                return Some(Vec::new());
            }
        };

        if track.module.version_code >= version_code {
            tracing::info!(
                target: "Update::check_versions",
                id = %module.id,
                "Already latest version"
            );

            None
        } else {
            track
                .versions
                .sort_by(|a, b| b.version_code.cmp(&a.version_code));

            inner();
            Some(track.versions)
        }
    }

    fn keep_size(&self, module: &Module) -> usize {
        let size = module.setting.keep_size.unwrap_or(self.setting.keep_size);

        tracing::debug!(target: "keep_size", id = %module.id, %size);
        size
    }

    fn remove_old(&self, module: &Module, old: &[Version]) {
        let module_dir = self.modules_dir.join(&module.id);

        old.iter().for_each(|v| {
            FileUtil::remove(module_dir.join(&v.zip_path));
            FileUtil::remove(module_dir.join(&v.changelog));
        })
    }

    fn write_track(&self, module: &Module, track: &Track) -> bool {
        let module_dir = self.modules_dir.join(&module.id);
        let track_json = module_dir.join(constant::TRACK_JSON);

        match track.to_file(track_json, true) {
            Ok(_) => true,
            Err(error) => {
                tracing::error!(target: "Update::write_track", id = %module.id, ?error);

                let version = &track.versions[0];
                FileUtil::remove(module_dir.join(&version.zip_path));
                FileUtil::remove(module_dir.join(&version.changelog));

                false
            }
        }
    }

    async fn update_common(
        &self,
        module: &Module,
        module_new: origin::Module,
        mut versions: Vec<Version>,
        timestamp: i64,
        changelog_url: &str,
    ) -> bool {
        let module_dir = self.modules_dir.join(&module.id);
        let zip_tmp = module_dir.join(constant::TMP_FILE);

        let version = StrUtil::get_version_display(&module_new.version, module_new.version_code);
        let mut version = Version::new(timestamp, version, module_new.version_code);
        FileUtil::rename(&zip_tmp, &module_dir.join(&version.zip_path));

        let changelog = module_dir.join(&version.changelog);
        let is_ok = Request::write_file(changelog_url, &changelog).await;
        if !is_ok || FileUtil::is_html(&changelog) {
            version.changelog = String::new();
            FileUtil::remove(&changelog);
        }

        versions.insert(0, version);
        let keep_size = self.keep_size(module);
        if versions.len() > keep_size {
            let old = versions.split_off(keep_size);
            self.remove_old(module, &old);
        }

        let track = Track {
            module: module_new,
            versions,
        };
        self.write_track(module, &track)
    }

    pub async fn update_by_json(&self, module: &Module) -> bool {
        let module_dir = self.modules_dir.join(&module.id);
        let mut timestamp = Utc::now().timestamp_millis();

        let update_json: UpdateJson = match Request::load_json(&module.provider).await {
            Some(u) => u,
            None => return false,
        };

        let versions = match self.check_versions(
            module, &update_json.version, update_json.version_code
        ) {
            Some(v) => v,
            None => return false,
        };

        let zip_tmp = module_dir.join(constant::TMP_FILE);
        match Request::new(&update_json.zip_url).await {
            Some(request) => {
                if let Some(last_modified) = request.last_modified() {
                    timestamp = last_modified.timestamp_millis();
                }

                if !request.write(&zip_tmp).await {
                    FileUtil::remove(&zip_tmp);
                    return false;
                }
            }
            None => return false,
        };

        let module_new = match LocalModule::read_zip(&zip_tmp) {
            Some(m) => m,
            None => {
                FileUtil::remove(&zip_tmp);
                return false;
            }
        };

        self.update_common(
            module,
            module_new,
            versions,
            timestamp,
            &update_json.changelog,
        ).await
    }

    pub async fn update_by_url(&self, module: &Module) -> bool {
        let module_dir = self.modules_dir.join(&module.id);
        let mut timestamp = Utc::now().timestamp_millis();

        let zip_tmp = module_dir.join(constant::TMP_FILE);
        match Request::new(&module.provider).await {
            Some(request) => {
                if let Some(last_modified) = request.last_modified() {
                    timestamp = last_modified.timestamp_millis();
                }

                if !request.write(&zip_tmp).await {
                    FileUtil::remove(&zip_tmp);
                    return false;
                }
            }
            None => return false,
        };

        let module_new = match LocalModule::read_zip(&zip_tmp) {
            Some(m) => m,
            None => {
                FileUtil::remove(zip_tmp);
                return false;
            }
        };

        let versions = match self.check_versions(
            module, &module_new.version, module_new.version_code
        ) {
            Some(v) => v,
            None => {
                FileUtil::remove(&zip_tmp);
                return false;
            }
        };

        self.update_common(
            module, 
            module_new, 
            versions, 
            timestamp, 
            &module.changelog
        ).await
    }

    #[cfg(feature = "git")]
    pub async fn update_by_git(&self, module: &Module) -> bool {
        use crate::util::Git;

        let module_dir = self.modules_dir.join(&module.id);
        let mut timestamp = Utc::now().timestamp_millis();

        let dir_tmp = module_dir.join(constant::TMP_DIR);
        match Git::try_clone(&module.provider, &dir_tmp).await {
            Some(t) => {
                timestamp = t.timestamp_millis();
            }
            None => {
                FileUtil::remove(&dir_tmp);
                return false;
            }
        }

        let zip_tmp = module_dir.join(constant::TMP_FILE);
        let module_new = match LocalModule::from_zip(&dir_tmp, &zip_tmp) {
            Some(module) => {
                FileUtil::remove(&dir_tmp);
                module
            }
            None => {
                FileUtil::remove(&dir_tmp);
                FileUtil::remove(&zip_tmp);
                return false;
            }
        };

        let versions = match self.check_versions(
            module, &module_new.version, module_new.version_code
        ) {
            Some(v) => v,
            None => {
                FileUtil::remove(&zip_tmp);
                return false;
            }
        };

        self.update_common(
            module, 
            module_new, 
            versions, 
            timestamp, 
            &module.changelog
        ).await
    }

    pub async fn update(&self, module: &Module) -> bool {
        tracing::debug!(target: "Update::update", ?module);
        if module.setting.disabled {
            return false;
        }

        tracing::info!(target: "Update::update", id = %module.id, kind = ?module.kind);
        match module.kind {
            ProviderKind::UpdateJson => self.update_by_json(module).await,
            ProviderKind::ZipUrl => self.update_by_url(module).await,
            #[cfg(feature = "git")]
            ProviderKind::Git => self.update_by_git(module).await,
        }
    }
}

pub struct UpdateWrapper<'u> {
    modules: &'u Vec<Arc<Module>>,
    original: Arc<Update>,
}

impl<'u> UpdateWrapper<'u> {
    pub fn build(context: &'u Context) -> Self {
        let update = Update::new(&context.repository.setting, &context.modules_dir);

        Self {
            modules: &context.modules,
            original: Arc::new(update),
        }
    }

    pub async fn update_all(&self, module_ids: &[String]) {
        let modules: Vec<Arc<Module>> = if module_ids.is_empty() {
            self.modules.iter().map(|m| m.to_owned()).collect()
        } else {
            self.modules
                .iter()
                .filter(|m| module_ids.contains(&m.id))
                .map(|m| m.to_owned())
                .collect()
        };

        let tasks: Vec<JoinHandle<bool>> = modules
            .into_iter()
            .map(|m| {
                let original = self.original.to_owned();
                tokio::spawn(async move { original.update(&m).await })
            })
            .collect();

        for task in tasks {
            task.await.ok();
        }
    }
}

impl Deref for UpdateWrapper<'_> {
    type Target = Update;

    fn deref(&self) -> &Self::Target {
        &self.original
    }
}

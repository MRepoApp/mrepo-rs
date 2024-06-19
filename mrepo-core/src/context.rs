use std::path::{Path, PathBuf};
use std::sync::Arc;

use mrepo_model::config::{Config, Log, Module, Repository};

use crate::error;
use crate::util::Json;

pub struct Context {
    pub log: Log,
    pub repository: Repository,
    pub modules: Vec<Arc<Module>>,
    pub config_path: PathBuf,
    pub json_dir: PathBuf,
    pub modules_dir: PathBuf,
}

impl Context {
    pub fn new<P: AsRef<Path>>(config_path: P, json_dir: P, modules_dir: P) -> error::Result<Self> {
        let config = Config::from_file(&config_path)?;

        Ok(Self {
            log: config.log,
            repository: config.repository,
            modules: config.modules.into_iter().map(Arc::new).collect(),
            config_path: PathBuf::from(config_path.as_ref()),
            json_dir: PathBuf::from(json_dir.as_ref()),
            modules_dir: PathBuf::from(modules_dir.as_ref()),
        })
    }
}

use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use mrepo_model::config::Log;

pub use crate::context::Context;
pub use crate::format::{Format, FormatWrapper};
pub use crate::update::{Update, UpdateWrapper};
pub use crate::upgrade::{Upgrade, UpgradeWrapper};

pub mod constant;
mod context;
pub mod error;
mod format;
mod update;
mod upgrade;
pub mod util;

pub struct ContextWrapper {
    original: Arc<Context>,
}

impl ContextWrapper {
    pub fn build<P: AsRef<Path>>(config_path: P, working_dir: P) -> error::Result<Self> {
        let config_path = config_path.as_ref().to_owned();
        let working_dir = working_dir.as_ref().to_owned();
        let json_dir = working_dir.join(constant::JSON_DIR);
        let modules_dir = working_dir.join(constant::MODULES_DIR);

        let context = Context::new(config_path, json_dir, modules_dir)?;
        Ok(Self {
            original: Arc::new(context),
        })
    }

    pub fn from_working_dir<P: AsRef<Path>>(working_dir: P) -> error::Result<Self> {
        let working_dir = working_dir.as_ref().to_owned();
        let json_dir = working_dir.join(constant::JSON_DIR);
        let modules_dir = working_dir.join(constant::MODULES_DIR);
        let config_path = json_dir.join(constant::CONFIG_JSON);

        let context = Context::new(config_path, json_dir, modules_dir)?;
        Ok(Self {
            original: Arc::new(context),
        })
    }

    pub fn logger<T, F: FnOnce(&Log) -> T>(&self, init: F) -> T {
        init(&self.log)
    }

    pub fn format(&self) -> FormatWrapper {
        FormatWrapper::build(self)
    }

    pub fn update(&self) -> UpdateWrapper {
        UpdateWrapper::build(self)
    }

    pub fn upgrade(&self) -> UpgradeWrapper {
        UpgradeWrapper::build(self)
    }
}

impl Deref for ContextWrapper {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.original
    }
}

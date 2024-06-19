use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use mrepo_model::config::Config;

use crate::context::Context;
use crate::util::Json;

pub struct Format {
    config_path: PathBuf,
}

impl Format {
    pub fn new<P: AsRef<Path>>(config_path: P) -> Self {
        Self {
            config_path: PathBuf::from(config_path.as_ref()),
        }
    }

    pub fn write_to<P: AsRef<Path>>(&self, path: P) -> bool {
        fn inner(from: &PathBuf, to: &Path) -> anyhow::Result<()> {
            let config = Config::from_file(from)?;
            config.to_file(to, true)?;
            Ok(())
        }

        let path = path.as_ref();
        match inner(&self.config_path, path) {
            Ok(_) => true,
            Err(error) => {
                tracing::error!(target: "Format::write_to", ?path, ?error);
                false
            }
        }
    }
}

pub struct FormatWrapper {
    original: Arc<Format>,
}

impl FormatWrapper {
    pub fn build(context: &Context) -> Self {
        let format = Format::new(&context.config_path);

        Self {
            original: Arc::new(format),
        }
    }

    pub fn write(&self) -> bool {
        self.write_to(&self.config_path)
    }
}

impl Deref for FormatWrapper {
    type Target = Format;

    fn deref(&self) -> &Self::Target {
        &self.original
    }
}

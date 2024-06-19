use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use walkdir::WalkDir;
use zip::{CompressionMethod, ZipArchive};
use zip::write::SimpleFileOptions;

use mrepo_model::origin::Module;

use crate::constant;

pub struct LocalModule;

impl LocalModule {
    pub fn read_prop<P: AsRef<Path>>(path: P) -> Option<Module> {
        fn inner(path: &Path) -> anyhow::Result<Module> {
            let bytes = fs::read(path)?;
            Ok(serde_prop::from_slice(&bytes)?)
        }

        let path = path.as_ref();
        if !path.exists() {
            return None;
        }

        tracing::debug!(target: "LocalModule::read_prop", ?path);
        match inner(path) {
            Ok(m) => Some(m),
            Err(error) => {
                tracing::error!(target: "LocalModule::read_prop", ?path, ?error);
                None
            }
        }
    }

    pub fn from_zip<P: AsRef<Path>>(from: P, to: P) -> Option<Module> {
        fn inner(from: &Path, to: &Path) -> anyhow::Result<()> {
            if let Some(parent) = to.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?
                }
            }

            let file = File::create(to)?;
            let mut zip = zip::ZipWriter::new(file);
            let options =
                SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

            let walk_dir = WalkDir::new(from);
            for entry in walk_dir.into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                let name = path.strip_prefix(from)?;
                let path_inner = match name.to_str() {
                    Some(p) => p.to_owned(),
                    None => continue,
                };

                if path_inner.starts_with('.') {
                    continue;
                }

                if path.is_file() {
                    let buffer = fs::read(path)?;
                    zip.start_file(path_inner, options)?;
                    zip.write_all(&buffer)?;
                } else if !path_inner.is_empty() {
                    zip.add_directory(path_inner, options)?;
                }
            }

            zip.finish()?;
            Ok(())
        }

        let from = from.as_ref();
        let to = to.as_ref();
        if !from.exists() {
            return None;
        }

        tracing::debug!(target: "LocalModule::from_zip", ?from, ?to);

        let prop_file = from.join(constant::MODULE_PROP);
        let module = Self::read_prop(prop_file)?;

        match inner(from, to) {
            Ok(_) => Some(module),
            Err(error) => {
                tracing::error!(target: "LocalModule::from_zip", ?from, ?error);
                None
            }
        }
    }

    pub fn read_zip<P: AsRef<Path>>(path: P) -> Option<Module> {
        fn inner(path: &Path) -> anyhow::Result<Module> {
            let file = File::open(path)?;
            let mut archive = ZipArchive::new(file)?;
            let mut zip = archive.by_name(constant::MODULE_PROP)?;

            let mut bytes = Vec::new();
            zip.read_to_end(&mut bytes)?;

            let module = serde_prop::from_slice(&bytes)?;
            Ok(module)
        }

        let path = path.as_ref();
        if !path.exists() {
            return None;
        }

        tracing::debug!(target: "LocalModule::read_zip", ?path);
        match inner(path) {
            Ok(m) => Some(m),
            Err(error) => {
                tracing::error!(target: "LocalModule::read_zip", ?path, ?error);
                None
            }
        }
    }
}
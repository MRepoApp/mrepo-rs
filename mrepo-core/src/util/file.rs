use std::fs;
use std::path::Path;
use crate::util::str::StrUtil;

pub struct FileUtil;

impl FileUtil {
    pub fn remove<P: AsRef<Path>>(path: P) -> bool {
        fn inner(path: &Path) -> anyhow::Result<()> {
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else if path.is_file() {
                fs::remove_file(path)?;
            }

            Ok(())
        }

        let path = path.as_ref();
        if !path.exists() {
            return true;
        }

        tracing::debug!(target: "FileUtil::remove", ?path);
        if let Err(error) = inner(path) {
            tracing::error!(target: "FileUtil::remove", ?path, ?error);
            false
        } else {
            true
        }
    }

    pub fn rename<P: AsRef<Path>>(from: P, to: P) -> bool {
        let from = from.as_ref();
        let to = to.as_ref();
        if !from.exists() {
            return false;
        }

        tracing::debug!(target: "FileUtil::rename", ?from, ?to);
        if let Err(error) = fs::rename(from, to) {
            tracing::error!(target: "FileUtil::rename", ?to, ?error);
            false
        } else {
            true
        }
    }

    #[inline]
    pub fn is_html<P: AsRef<Path>>(path: P) -> bool {
        if let Ok(text) = fs::read_to_string(path) {
            StrUtil::is_html(&text)
        } else {
            false
        }
    }
}

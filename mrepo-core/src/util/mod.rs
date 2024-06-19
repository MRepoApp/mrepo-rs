use std::fs;
use std::fs::File;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub use file::FileUtil;
#[cfg(feature = "git")]
pub use git::Git;
pub use module::LocalModule;
pub use request::Request;
pub use str::StrUtil;

use crate::error;
use crate::error::Error;

mod file;
pub mod str;
mod request;
mod module;
#[cfg(feature = "git")]
mod git;

pub trait Json: Sized {
    fn from_file<P: AsRef<Path>>(p: P) -> error::Result<Self>;
    fn from_slice(v: &[u8]) -> error::Result<Self>;
    fn to_file<P: AsRef<Path>>(&self, p: P, pretty: bool) -> error::Result<()>;
    fn to_string(&self) -> error::Result<String>;
    fn to_string_pretty(&self) -> error::Result<String>;
}

impl<T: DeserializeOwned + Serialize> Json for T {
    fn from_file<P: AsRef<Path>>(p: P) -> error::Result<Self> {
        let v = fs::read(p).map_err(Error::io)?;
        let value = Self::from_slice(&v)?;
        Ok(value)
    }

    #[inline]
    fn from_slice(v: &[u8]) -> error::Result<Self> {
        serde_json::from_slice(v).map_err(Error::json)
    }

    fn to_file<P: AsRef<Path>>(&self, p: P, pretty: bool) -> error::Result<()> {
        let file = File::create(p).map_err(Error::io)?;
        if pretty {
            serde_json::to_writer_pretty(file, self)
        } else {
            serde_json::to_writer(file, self)
        }
        .map_err(Error::json)?;

        Ok(())
    }

    #[inline]
    fn to_string(&self) -> error::Result<String> {
        serde_json::to_string(self).map_err(Error::json)
    }

    #[inline]
    fn to_string_pretty(&self) -> error::Result<String> {
        serde_json::to_string_pretty(self).map_err(Error::json)
    }
}

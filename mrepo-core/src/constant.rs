#![allow(unused)]

pub const CONFIG_JSON: &str = "config.json";
pub const MODULES_JSON: &str = "modules.json";
pub const TRACK_JSON: &str = "track.json";
pub const JSON_DIR: &str = "json";
pub const MODULES_DIR: &str = "modules";

pub(crate) const MODULE_PROP: &str = "module.prop";
pub(crate) const TMP_FILE: &str = "tmp";
pub(crate) const TMP_DIR: &str = "tmp.d";

#[cfg(feature = "git")]
pub const SSH_PRIVATE_KEY: &str = "SSH_PRIVATE_KEY";

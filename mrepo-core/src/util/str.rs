use once_cell::sync::Lazy;
use regex::Regex;

pub struct StrUtil;

static HTML_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"<html\s*>|<head\s*>|<body\s*>|<!doctype\s*html\s*>").unwrap());

impl StrUtil {
    #[inline]
    pub fn is_html(text: &str) -> bool {
        HTML_RE.find(text).is_some()
    }

    pub fn get_version_display(version: &str, version_code: i64) -> String {
        let re = Regex::new(&format!(r"\(.*?{version_code}.*?\)")).unwrap();
        if re.find(version).is_some() {
            version.to_string()
        } else {
            format!("{version} ({version_code})")
        }
    }
}
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use chrono::{DateTime, Utc};
use reqwest::{Response, StatusCode};
use reqwest::header::{HeaderMap, LAST_MODIFIED};

use crate::util::Json;

pub struct Request {
    url: String,
    response: Response,
}

impl Request {
    pub async fn new(url: &str) -> Option<Self> {
        if url.is_empty() {
            return None;
        }

        match reqwest::get(url).await {
            Ok(response) => {
                let status = response.status();
                if status == StatusCode::OK {
                    return Some(Self {
                        url: url.to_owned(),
                        response,
                    });
                }

                tracing::error!(target: "Request::new", %url, %status);
                None
            }
            Err(error) => {
                tracing::error!(target: "Request::new", %url, ?error);
                None
            }
        }
    }

    #[inline]
    pub async fn write_file<P: AsRef<Path>>(url: &str, path: P) -> bool {
        match Self::new(url).await {
            Some(request) => request.write(path).await,
            None => false,
        }
    }

    #[inline]
    pub async fn load_json<T: Json>(url: &str) -> Option<T> {
        match Self::new(url).await {
            Some(request) => request.json().await,
            None => None,
        }
    }

    pub fn headers(&self) -> &HeaderMap {
        self.response.headers()
    }

    pub fn last_modified(&self) -> Option<DateTime<Utc>> {
        if let Some(last_modified) = self.headers().get(LAST_MODIFIED) {
            if let Ok(last_modified) = last_modified.to_str() {
                DateTime::parse_from_rfc2822(last_modified)
                    .map(|t| t.to_utc())
                    .ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn write<P: AsRef<Path>>(self, path: P) -> bool {
        async fn inner(response: Response, path: &Path) -> anyhow::Result<()> {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?
                }
            }

            let bytes = response.bytes().await?;
            let mut file = File::create(path)?;
            file.write_all(&bytes)?;
            Ok(())
        }

        let path = path.as_ref();

        tracing::debug!(target: "Request::write", ?path, url = %self.url);
        match inner(self.response, path).await {
            Ok(_) => true,
            Err(error) => {
                tracing::error!(target: "Request::write", ?path, url = %self.url, ?error);
                false
            }
        }
    }

    pub async fn json<T: Json>(self) -> Option<T> {
        async fn inner<T: Json>(response: Response) -> anyhow::Result<T> {
            let bytes = response.bytes().await?;
            Ok(T::from_slice(&bytes)?)
        }

        match inner(self.response).await {
            Ok(t) => Some(t),
            Err(error) => {
                tracing::error!(target: "Request::json", url = %self.url, ?error);
                None
            }
        }
    }
}
use std::path::Path;
use std::{env, fs};

use chrono::{DateTime, Utc};
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use once_cell::sync::Lazy;

use crate::constant;

pub struct Git;

static SSH_PRIVATE_KEY: Lazy<String> = Lazy::new(|| {
    env::var(constant::SSH_PRIVATE_KEY).ok().unwrap_or_else(|| {
        tracing::warn!(target: "Git::new", "No SSH key was provided, you can set environment variable `SSH_PRIVATE_KEY`");
        String::new()
    })
});

impl<'g> Git {
    pub fn commit_time(repository: &Repository) -> Option<DateTime<Utc>> {
        let commit = match repository.head() {
            Ok(reference) => reference.peel_to_commit().unwrap(),
            Err(error) => {
                tracing::error!(target: "Git::commit_time", path = ?repository.path(), ?error);
                return Some(Utc::now());
            }
        };

        DateTime::from_timestamp(commit.time().seconds(), 0)
    }

    #[inline]
    pub async fn try_clone<P: AsRef<Path>>(url: &str, path: P) -> Option<DateTime<Utc>> {
        match Self::clone(url, path).await {
            Some(r) => Self::commit_time(&r),
            None => None,
        }
    }

    fn builder() -> RepoBuilder<'g> {
        let mut callbacks = RemoteCallbacks::new();
        if !SSH_PRIVATE_KEY.is_empty() {
            callbacks.credentials(|_url, username, _types| {
                Cred::ssh_key_from_memory(username.unwrap(), None, &SSH_PRIVATE_KEY, None)
            });
        }

        let mut options = FetchOptions::new();
        options.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(options);
        builder
    }

    pub async fn clone<P: AsRef<Path>>(url: &str, path: P) -> Option<Repository> {
        fn inner(url: &str, path: &Path) -> anyhow::Result<Repository> {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?
                }
            }

            Ok(Git::builder().clone(url, path)?)
        }

        if url.is_empty() {
            return None;
        }

        let path = path.as_ref();
        tracing::debug!(target: "Git::clone", %url, ?path);

        match inner(url, path) {
            Ok(r) => Some(r),
            Err(error) => {
                tracing::error!(target: "Git::clone", %url, ?path, ?error);
                None
            }
        }
    }
}

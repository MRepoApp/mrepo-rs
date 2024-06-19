use std::path::PathBuf;
use std::env;

use clap::Parser;
use mrepo_cli::{Args, Commands};
use mrepo_core::ContextWrapper;

#[inline]
fn get_working_dir(args: &Args) -> Option<PathBuf> {
    match &args.directory {
        Some(d) => Some(d.to_owned()),
        None => env::current_dir().ok(),
    }
}

#[cfg(feature = "git")]
fn set_ssh_key(key: Option<String>) {
    use std::fs;
    use mrepo_core::constant;
    
    if env::var(constant::SSH_PRIVATE_KEY).is_ok() {
        return;
    }

    if let Some(value) = key {
        if value.contains("BEGIN OPENSSH PRIVATE KEY") {
            env::set_var(constant::SSH_PRIVATE_KEY, value);
            return;
        }

        if let Ok(key) = fs::read_to_string(value) {
            env::set_var(constant::SSH_PRIVATE_KEY, key);
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let working_dir = get_working_dir(&args);
    let working_dir = match working_dir {
        Some(d) => d,
        None => {
            eprintln!("Working directory not found!");
            return;
        }
    };

    let context = match args.config {
        Some(config_path) => ContextWrapper::build(config_path, working_dir),
        None => ContextWrapper::from_working_dir(working_dir),
    };
    let context = match context {
        Ok(c) => c,
        Err(error) => {
            eprintln!("Failed to create context: {error}");
            return;
        }
    };

    let _logger: Option<_> = if !args.quiet {
        Some(context.logger(mrepo_log::init_tracing))
    } else {
        None
    };

    match args.command {
        Commands::Format { write } => {
            let format = context.format();
            match write {
                Some(path) => format.write_to(path),
                None => format.write(),
            };
        }
        #[cfg(feature = "git")]
        Commands::Update { id, ssh_key } => {
            let id = id.unwrap_or(Vec::new());
            let update = context.update();
            set_ssh_key(ssh_key);
            update.update_all(&id).await;
        }
        #[cfg(not(feature = "git"))]
        Commands::Update { id } => {
            let id = id.unwrap_or(Vec::new());
            let update = context.update();
            update.update_all(&id).await;
        }
        Commands::Upgrade { write, pretty } => {
            let upgrade = context.upgrade();
            match write {
                Some(path) => upgrade.generate_index_to(path, pretty).await,
                None => upgrade.generate_index(pretty).await,
            };
        }
    };
}

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, disable_colored_help = true, disable_help_subcommand = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    /// Set config file path
    #[arg(short, long, value_name = "PATH", default_value = None, global = true)]
    pub config: Option<PathBuf>,

    /// Set working directory
    #[arg(short = 'D', long, value_name = "DIR", default_value = None, global = true)]
    pub directory: Option<PathBuf>,

    /// Do not print log messages
    #[arg(long, default_value = "false", global = true)]
    pub quiet: bool,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Format configuration
    Format {
        /// Write formatted config to file
        #[arg(short, long, value_name = "PATH", default_value = None)]
        write: Option<PathBuf>,
    },

    /// Update modules
    Update {
        /// All by default
        #[arg(default_value = None)]
        id: Option<Vec<String>>,

        /// Set SSH key
        #[cfg(feature = "git")]
        #[arg(long, value_name = "PATH|KEY", default_value = None)]
        ssh_key: Option<String>,
    },

    /// Upgrade index
    Upgrade {
        /// Write index to file
        #[arg(short, long, value_name = "PATH", default_value = None)]
        write: Option<PathBuf>,

        /// Write as pretty-printed
        #[arg(long, default_value = "false")]
        pretty: bool,
    },
}

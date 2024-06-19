use std::path::Path;
use std::str::FromStr;

use tracing::Level;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

use mrepo_model::config::Log;

pub fn init_tracing(log: &Log) -> Option<WorkerGuard> {
    macro_rules! init {
        ($log:expr, $subscriber:expr) => {
            if $log.timestamp {
                $subscriber.init()
            } else {
                $subscriber.without_time().init()
            }
        };
    }

    fn non_blocking(path: &Path) -> Option<(NonBlocking, WorkerGuard)> {
        let log_dir = path.parent()?;
        let file_name = path.file_name()?;

        let file_appender = tracing_appender::rolling::never(log_dir, file_name);
        Some(tracing_appender::non_blocking(file_appender))
    }

    let level = Level::from_str(&log.level).unwrap_or(Level::INFO);
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(true);

    if log.output.is_empty() {
        let subscriber = subscriber.with_ansi(true);
        init!(log, subscriber);
        None
    } else {
        let path = Path::new(&log.output);
        if let Some((log_file, guard)) = non_blocking(path) {
            let subscriber = subscriber.with_ansi(false).with_writer(log_file);
            init!(log, subscriber);
            Some(guard)
        } else {
            let subscriber = subscriber.with_ansi(true);
            init!(log, subscriber);
            None
        }
    }
}

use std::path::Path;
use std::str::FromStr;

use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;

use mrepo_model::config::Log;

pub struct Logger<T> {
    _guard: Option<T>,
}

impl<T> Logger<T> {
    pub fn new(guard: Option<T>) -> Self {
        Self { _guard: guard }
    }
}

impl Logger<WorkerGuard> {
    pub fn init_tracing(log: &Log) -> Self {
        if log.disabled {
            return Logger::new(None);
        }

        let level = Level::from_str(&log.level).unwrap_or(Level::INFO);
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(level)
            .with_target(true);

        let guard = if log.output.is_empty() {
            let subscriber = subscriber.with_ansi(true);
            if log.timestamp {
                subscriber.init()
            } else {
                subscriber.without_time().init()
            }

            None
        } else {
            let path = Path::new(&log.output);
            let log_dir = path.parent().unwrap();
            let file_name = path.file_name().unwrap();

            let file_appender = tracing_appender::rolling::never(log_dir, file_name);
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
            let subscriber = subscriber.with_ansi(false).with_writer(non_blocking);
            if log.timestamp {
                subscriber.init()
            } else {
                subscriber.without_time().init()
            }

            Some(guard)
        };

        Self::new(guard)
    }
}

pub fn init_tracing(log: &Log) -> Logger<WorkerGuard> {
    Logger::init_tracing(log)
}

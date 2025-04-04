//!
//! # logs
//!

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// setup logger.
///
pub fn setup_logger(logfilepath: &std::path::Path) -> anyhow::Result<WorkerGuard> {
    let logdir = logfilepath.parent().unwrap();
    if !logdir.exists() {
        std::fs::create_dir_all(logdir)?;
    }

    let logfile = logfilepath.file_name().unwrap();

    let fa = tracing_appender::rolling::never(logdir, logfile);
    let (non_blocking, guard) = tracing_appender::non_blocking(fa);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::Layer::default().with_writer(non_blocking))
        .init();

    Ok(guard)
}

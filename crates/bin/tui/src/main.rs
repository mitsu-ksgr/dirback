//!
//! Dirback - Text user interface.
//!

use dirback::infra::app_path;
use tracing::info;

mod logs;
mod options;

fn get_basedir_path() -> anyhow::Result<std::path::PathBuf> {
    // If DIRBACK_STORE_DIR is set, use it's value.
    // If not set, use app_path::data_dir.
    let basedir = std::env::var("DIRBACK_STORE_DIR")
        .ok()
        .map(std::path::PathBuf::from)
        .or_else(app_path::data_dir)
        .ok_or_else(|| anyhow::anyhow!("Failed to get path to directory for application data."))?;

    Ok(basedir)
}

fn main() {
    let opts = options::parse_args();

    let basedir = match opts.datadir {
        Some(datadir) => datadir.clone(),
        None => get_basedir_path().unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }),
    };

    let logfile = match opts.logfile {
        Some(logfile) => logfile.clone(),
        None => basedir.join("dirback.log"),
    };

    let _guard = logs::setup_logger(&logfile).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    info!("Start Dirback TUI");
    info!("Dirback base dir: {}", basedir.display());
    info!("Log file: {}", logfile.display());

    if let Err(ref e) = dirback_tui::run(&basedir) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

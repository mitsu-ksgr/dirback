//!
//! # dirback gui
//!

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::info;

mod logs;
mod options;

fn main() {
    let opts = match options::Options::build() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let _guard = logs::setup_logger(&opts.logfile).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    info!("Start Dirback GUI");
    info!("Dirback base dir: {}", opts.datadir.display());
    info!("Log file: {}", opts.logfile.display());

    if let Err(ref e) = dirback_gui_lib::run(&opts.datadir) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

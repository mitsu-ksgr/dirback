//!
//! Dirback - Text user interface.
//!

use dirback::infra::app_path;

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
    let args: Vec<String> = std::env::args().collect();
    let basedir = get_basedir_path().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    println!("BaseDir: {basedir:?}");

    if let Err(ref e) = dirback_tui::run(&basedir) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

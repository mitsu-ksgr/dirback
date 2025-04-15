//!
//! # CLI arguments options
//!

use dirback::infra::app_path;

pub struct Options {
    /// Sets a custom dirback data directory.
    pub datadir: std::path::PathBuf,

    /// Sets a custom log file path.
    /// The default path is `{datadir}/dirback.log`.
    pub logfile: std::path::PathBuf,
}

impl Options {
    pub fn build() -> anyhow::Result<Self> {
        let basedir = get_basedir_path()?;

        Ok(Self {
            datadir: basedir.clone(),
            logfile: basedir.join("dirback.log"),
        })
    }
}

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

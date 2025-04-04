//!
//! # CLI arguments options
//!

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
    /// Sets a custom dirback data directory.
    #[arg(short, long, value_name = "DIRBACK_STORE_DIR")]
    pub datadir: Option<std::path::PathBuf>,

    /// Sets a custom log file path.
    /// The default path is `{datadir}/dirback.log`.
    #[arg(short, long, value_name = "FILE")]
    pub logfile: Option<std::path::PathBuf>,
}

pub fn parse_args() -> Options {
    Options::parse()
}

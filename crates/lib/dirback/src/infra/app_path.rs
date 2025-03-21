//!
//! # App Dirs
//!
//! Provide paths for the applicaiton.
//!

use std::path::PathBuf;

/*
struct InfoPath(std::path::PathBuf);

impl InfoPath {
    fn from_target_id(target_id: &str) -> Self {
        //let app_dir = directories::ProjectDirs::from("", "", "HogeApp").unwrap();
        Self(std::path::PathBuf::new(""))
    }
}
*/

/// Returns the data directory path for the application.
pub fn data_dir() -> Option<PathBuf> {
    use directories::ProjectDirs;
    let pkgname = env!("CARGO_PKG_NAME");

    // https://docs.rs/directories/6.0.0/directories/struct.ProjectDirs.html
    ProjectDirs::from("", "", pkgname).map(|pd| pd.data_dir().to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datadir_path_ends_with_pakcage_name() {
        let appname = env!("CARGO_PKG_NAME");

        let result = data_dir();
        assert!(result.is_some());

        let dir = result.unwrap();
        assert!(dir.to_string_lossy().ends_with(appname));

        println!("AppName   : {appname}");
        println!("Data Path : {}", dir.to_string_lossy());
    }
}

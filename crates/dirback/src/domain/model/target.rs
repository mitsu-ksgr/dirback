//!
//! # Target
//!
//! Backup target.
//! Contains the target directory path and id.
//!
//!

use crate::domain::model::backup_file::BackupFile;

#[derive(Debug)]
pub struct Target {
    /// Target ID.
    ///
    /// Used as an id when saving backups.
    pub id: String,

    /// Path to the target dir.
    pub path: PathBuf,

    /// Backup files.
    pub backups: Vec<BackupFile>,
}

impl Target {
    /// Create a new BackupTarget.
    ///
    /// The id is the identifier of target.
    pub fn new(id: String, target_dir_path: &Path) -> Self {
        Target {
            id,
            path: target_dir_path.to_path_buf(),
            backups: Vec::<BackupFile>::new(),
        }
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let td = Target::new(String::from("xxx"), String::from("/path/to/target"));

        assert_eq!(td.id, "xxx");
        assert_eq!(td.path, "/path/to/target");
    }
}

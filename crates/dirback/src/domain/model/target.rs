//!
//! # Target
//!
//! Target is the backup target.
//!
//! This module does not provide backup/restore methods.
//!

use crate::domain::model::backup_entry::BackupEntry;
use crate::domain::model::timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TargetError {
    #[error("Duplicate ID")]
    DuplicateId,
}

/// Target struct represents a backup target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Target {
    /// Target ID.
    ///
    /// Used as an id when saving target information and backups.
    pub id: String,

    /// Target Name.
    pub name: String,

    /// Path to the directory of backup target.
    pub path: PathBuf,

    /// Backup entries.
    pub backups: Vec<BackupEntry>,
}

impl Target {
    /// Create a new BackupTarget.
    ///
    /// The id is the identifier of target.
    pub fn new(id: &str, name: &str, target_dir_path: &Path) -> Self {
        Target {
            id: id.to_string(),
            name: name.to_string(),
            path: target_dir_path.to_path_buf(),
            backups: Vec::<BackupEntry>::new(),
        }
    }

    /// Make a new backup entry.
    ///
    /// The following parameters are set automatically.
    /// - id
    /// - timestamp
    ///
    /// The backup path derived from the arguments.
    ///
    /// # Arguments
    /// - `backup_dir` ... Backup base directory.
    /// - `ext` ... The file extension of the backup file.
    ///     - example: `tar.gz`, `zip`
    pub fn new_backup_entry(&self, backup_dir: &Path, ext: &str) -> BackupEntry {
        let next_id = self.backups.last().map_or(1, |last| last.id + 1);
        let now = Timestamp::now();

        let mut path = backup_dir.to_path_buf();
        path.push(BackupEntry::generate_backup_filename(next_id, &now, ext));

        BackupEntry::new(next_id, &path, now, "")
    }

    /// Add a backup entry.
    pub fn register_backup_entry(&mut self, entry: BackupEntry) -> Result<(), TargetError> {
        if let Some(_) = self.backups.iter().find(|&b| b.id == entry.id) {
            return Err(TargetError::DuplicateId);
        }

        self.backups.push(entry);
        Ok(())
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn prepare_target() -> Target {
        let target_id = Uuid::new_v4();
        let target_path = Path::new("/tmp/path/to/target-dir");
        Target::new(&target_id.to_string(), "Test Target", target_path)
    }

    fn prepare_backup_dir(target: &Target) -> PathBuf {
        PathBuf::from(&format!("/tmp/dirback/targets/{}/backups", target.id))
    }

    #[test]
    fn test_new() {
        let id = Uuid::new_v4();
        let target_path_str = String::from("/tmp/path/to/target-dir");
        let target_path = Path::new(&target_path_str);

        let target = Target::new(&id.to_string(), "Test Target", target_path);
        assert_eq!(target.id, id.to_string());
        assert_eq!(target.path, target_path.to_path_buf());
        assert_eq!(target.backups.len(), 0);
    }

    mod test_new_backup_entry {
        use super::*;

        #[test]
        fn it_works() {
            let target = prepare_target();
            let backup_dir = prepare_backup_dir(&target);

            let entry = target.new_backup_entry(&backup_dir, "tar.gz");

            assert_eq!(entry.id, 1);
            assert_eq!(entry.note, "");

            let fname = entry.path.file_name().and_then(|s| s.to_str()).unwrap();
            assert!(fname.starts_with("0001"));
        }

        #[test]
        fn id_will_be_same_before_register() {
            let target = prepare_target();
            let backup_dir = prepare_backup_dir(&target);

            let entry1 = target.new_backup_entry(&backup_dir, "tar.gz");
            let entry2 = target.new_backup_entry(&backup_dir, "tar.gz");

            assert_eq!(entry1.id, entry2.id);
        }

        #[test]
        fn id_auto_increment() {
            let mut target = prepare_target();
            let backup_dir = prepare_backup_dir(&target);
            assert_eq!(target.backups.len(), 0);

            let entry1 = target.new_backup_entry(&backup_dir, "tar.gz");
            let _ = target.register_backup_entry(entry1.clone());

            let entry2 = target.new_backup_entry(&backup_dir, "tar.gz");
            let _ = target.register_backup_entry(entry2.clone());

            assert_ne!(entry1.id, entry2.id);
            assert_eq!(target.backups.len(), 2);
        }

        #[test]
        fn id_is_next_of_last_entry() {
            let mut target = prepare_target();
            let backup_dir = prepare_backup_dir(&target);
            assert_eq!(target.backups.len(), 0);

            let mut entry1 = target.new_backup_entry(&backup_dir, "tar.gz");
            entry1.id = 5;
            let _ = target.register_backup_entry(entry1.clone());

            let entry2 = target.new_backup_entry(&backup_dir, "tar.gz");
            assert_eq!(entry2.id, entry1.id + 1);
        }
    }

    mod test_register_backup_entry {
        use super::*;

        #[test]
        fn it_works() {
            let mut target = prepare_target();
            let backup_dir = prepare_backup_dir(&target);

            let entry = target.new_backup_entry(&backup_dir, "tar.gz");

            assert_eq!(target.backups.len(), 0);
            let result = target.register_backup_entry(entry);
            assert!(result.is_ok());
            assert_eq!(target.backups.len(), 1);
        }

        #[test]
        fn error_duplicated_id() {
            let mut target = prepare_target();
            let backup_dir = prepare_backup_dir(&target);

            let entry1 = target.new_backup_entry(&backup_dir, "tar.gz");
            let _ = target.register_backup_entry(entry1.clone());

            let mut entry2 = target.new_backup_entry(&backup_dir, "tar.gz");
            entry2.id = entry1.id;

            let result = target.register_backup_entry(entry2.clone());
            assert!(result.is_err());
            assert_eq!(result, Err(TargetError::DuplicateId));
            assert_eq!(target.backups.len(), 1);
        }
    }

    #[test]
    fn it_serializable() {
        let mut src = prepare_target();
        let backup_dir = prepare_backup_dir(&src);

        for _ in 0..3 {
            let entry = src.new_backup_entry(&backup_dir, "tar.gz");
            let _ = src.register_backup_entry(entry.clone());
        }

        let s = serde_json::to_string(&src);
        assert!(s.is_ok(), "it should be serializable into json.");

        println!("{:?}", s);

        let dst = serde_json::from_str(&s.unwrap());
        assert!(dst.is_ok(), "it should be deserializable from json.");

        let dst: Target = dst.unwrap();
        assert_eq!(dst, src);
    }
}

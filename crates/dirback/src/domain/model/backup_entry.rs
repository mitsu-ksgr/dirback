//!
//! # BackupEntry
//!
//! BackupEntry represents the information of a backup.
//!

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::domain::model::timestamp::Timestamp;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupEntry {
    pub id: u32,
    pub path: PathBuf,
    pub timestamp: Timestamp,
    pub note: String,
}

impl BackupEntry {
    pub fn new(id: u32, path: &Path, timestamp: Timestamp, note: &str) -> Self {
        Self {
            id,
            path: path.to_path_buf(),
            timestamp,
            note: note.to_string(),
        }
    }

    /// Generates a filename for the backup file from the ID and a timestamp.
    ///
    /// ## Arguments
    /// - id ... Backup id
    /// - timestamp ... Backup timestamp
    /// - ext ... Backup file extension without "."
    pub fn generate_backup_filename(id: u32, timestamp: &Timestamp, ext: &str) -> String {
        format!("{:0>4}_{}.{}", id, timestamp.fmt(), ext).to_string()
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ts_str = "20250123T123456Z";
        let ts = Timestamp::from_fmt_str(ts_str).unwrap();
        let bkpath_str = format!("/tmp/dirback/{ts_str}.tar.gz");
        let bkpath = Path::new(&bkpath_str);
        let note = String::from("this is test backup file.");

        let entry = BackupEntry::new(1, bkpath, ts.clone(), &note);

        assert_eq!(entry.id, 1);
        assert_eq!(entry.path, bkpath.to_path_buf());
        assert_eq!(entry.timestamp, ts);
        assert_eq!(entry.note, note);
    }

    #[test]
    fn test_backup_filename() {
        let id = 15;
        let ts = Timestamp::from_fmt_str("20250123T123456Z").unwrap();
        let ext = "tar.gz";

        let result = BackupEntry::generate_backup_filename(id, &ts, ext);
        assert_eq!(result, "0015_20250123T123456Z.tar.gz");
        assert!(result.ends_with(ext));
    }

    #[test]
    fn it_serializable() {
        let id = 23;
        let ts = Timestamp::from_fmt_str("20250123T123456Z").unwrap();
        let bkpath_parts = [
            "tmp",
            "targets",
            "xxx",
            "backups",
            "023_20250123T123456Z.tar.gz",
        ];
        let bkpath: PathBuf = bkpath_parts.iter().collect();
        let note = "this is test backup file.";

        let src = BackupEntry::new(id, &bkpath, ts.clone(), &note);

        let s = serde_json::to_string(&src);
        assert!(s.is_ok(), "it should be serializable into json.");

        let dst = serde_json::from_str(&s.unwrap());
        assert!(dst.is_ok(), "it should be deserializable from json.");

        let dst: BackupEntry = dst.unwrap();
        assert_eq!(dst, src);
    }
}

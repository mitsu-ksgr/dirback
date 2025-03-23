//!
//! # BackupEntry DTO
//!

use crate::domain::model;
use crate::usecase::dto::Timestamp;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct BackupEntry {
    pub id: u32,
    pub path: PathBuf,
    pub timestamp: Timestamp,
    pub note: String,
}

impl std::convert::From<model::BackupEntry> for BackupEntry {
    fn from(entry: model::BackupEntry) -> Self {
        Self {
            id: entry.id,
            path: entry.path,
            timestamp: entry.timestamp,
            note: entry.note,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backupentry_to_dto_conversion() {
        let ts_str = "20250123T123456Z";
        let ts = Timestamp::from_fmt_str(ts_str).unwrap();
        let bkpath_str = format!("/tmp/dirback/{ts_str}.tar.gz");
        let bkpath = std::path::Path::new(&bkpath_str);
        let note = String::from("this is test backup file.");

        let entry = model::BackupEntry::new(1, bkpath, ts.clone(), &note);
        let dto: BackupEntry = entry.into();

        assert_eq!(dto.id, 1);
        assert_eq!(dto.path, bkpath);
        assert_eq!(dto.timestamp, ts);
        assert_eq!(dto.note, "this is test backup file.");
    }
}

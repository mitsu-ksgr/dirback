//!
//! # Target DTO
//!

use crate::domain::model;
use crate::usecase::dto::BackupEntry;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Target {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub backups: Vec<BackupEntry>,
}

impl std::convert::From<model::Target> for Target {
    fn from(target: model::Target) -> Self {
        Self {
            id: target.id,
            name: target.name,
            path: target.path,
            backups: target.backups.into_iter().map(BackupEntry::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_to_dto_conversion() {
        let target_id = String::from("xxxxxx");
        let target_path = std::path::Path::new("target-dir");
        let mut target = model::Target::new(&target_id.to_string(), "Test Target", target_path);

        let bkdir = PathBuf::from(&format!("targets/{}/backups", target.id));
        for _ in 1..=3 {
            let entry = target.new_backup_entry(&bkdir, "tar.gz");
            let _ = target.register_backup_entry(entry);
        }

        let dto: Target = target.into();

        assert_eq!(dto.id, target_id);
        assert_eq!(dto.name, "Test Target");
        assert_eq!(dto.path, target_path);
        assert_eq!(dto.backups.len(), 3);
    }
}

//!
//! # DeleteBackup command
//!

use crate::commands::Command;

use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::usecase::dto::BackupEntry;
use dirback::usecase::delete_backup::DeleteBackupUsecase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteBackupPayload {
    pub target_id: String,
    pub backup_id: u32,
}

pub struct DeleteBackup;

impl Command for DeleteBackup {
    type Payload = DeleteBackupPayload;
    type Output = BackupEntry;

    fn execute(
        &self,
        datadir: &std::path::Path,
        payload: Self::Payload,
    ) -> anyhow::Result<Self::Output> {
        let mut repo = FileStorageTargetRepository::new(datadir);
        let mut usecase = DeleteBackupUsecase::new(&mut repo);
        let entry = usecase.execute(&payload.target_id, payload.backup_id)?;
        Ok(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirback::infra::repository::file_storage::FileStorageTargetRepository;
    use dirback::internal::TargetRepository;
    use dirback::usecase::dto::Target;

    fn prepare_test_data(temp: &mktemp::TempDir) -> Target {
        let mut repo = FileStorageTargetRepository::new(&temp.path());
        let mut target = repo.add("TestTarget", std::path::Path::new(".")).unwrap();

        // backups
        let bkdir = repo.make_backup_dir_path(&target);
        for _ in 1..=3 {
            let entry = target.new_backup_entry(&bkdir, "tar.gz");
            let _ = std::fs::File::create(&entry.path); // make dummy backup.
            let _ = target.register_backup_entry(entry);
        }
        let target = repo.update(&target).unwrap();

        target.into()
    }

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();
        let repo = FileStorageTargetRepository::new(&basedir);

        // Test target
        let target = prepare_test_data(&temp);
        let before_backup_count = target.backups.len();

        // Command
        let cmd = DeleteBackup;
        let payload = DeleteBackupPayload {
            target_id: target.id.clone(),
            backup_id: 1,
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_ok());

        let got = result.unwrap();
        assert_eq!(got.id, 1);

        let after_target = repo.load(&target.id).unwrap();
        assert_eq!(after_target.backups.len(), before_backup_count - 1);
        assert!(
            !after_target.backups.iter().any(|be| be.id == 1),
            "target should not contain deleted backup entry.");
    }

    #[test]
    fn it_returns_err_when_target_not_found() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Command
        let cmd = DeleteBackup;
        let payload = DeleteBackupPayload {
            target_id: String::from("xxxxx-xxxxx-xxxxx"),
            backup_id: 1,
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_err());
    }

    #[test]
    fn it_returns_err_when_backup_not_found() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Test target
        let mut repo = FileStorageTargetRepository::new(&basedir);
        let target = repo.add("TestTarget", std::path::Path::new(".")).unwrap();

        // Command
        let cmd = DeleteBackup;
        let payload = DeleteBackupPayload {
            target_id: target.id,
            backup_id: 1,
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_err());
    }
}


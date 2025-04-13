//!
//! # RestoreTarget command
//!

use crate::commands::Command;

use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::infra::service::targz_backup_service::TargzBackupService;
use dirback::usecase::restore::RestoreUsecase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RestoreTargetPayload {
    pub target_id: String,
    pub backup_id: u32,
}

pub struct RestoreTarget;

impl Command for RestoreTarget {
    type Payload = RestoreTargetPayload;
    type Output = ();

    fn execute(
        &self,
        datadir: &std::path::Path,
        payload: Self::Payload,
    ) -> anyhow::Result<Self::Output> {
        let mut repo = FileStorageTargetRepository::new(datadir);
        let service = TargzBackupService::new();

        let mut usecase = RestoreUsecase::new(&mut repo, &service);
        usecase.execute(&payload.target_id, payload.backup_id)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirback::infra::repository::file_storage::FileStorageTargetRepository;
    use dirback::internal::TargetRepository;
    use dirback::usecase::backup::BackupUsecase;

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path().join("dirback");
        let _ = std::fs::create_dir_all(&basedir);
        let mut repo = FileStorageTargetRepository::new(&basedir);

        // Test target
        let targetdir = temp.path().join("test-target");
        let _ = std::fs::create_dir_all(&targetdir);
        let testfile = targetdir.join("test.txt");
        let _ = std::fs::File::create(&testfile);
        let target = repo.add("TestTarget", &targetdir).unwrap();

        // Backup
        let bk_service = TargzBackupService::new();
        let mut bk_usecase = BackupUsecase::new(&mut repo, &bk_service);
        let _ = bk_usecase.execute(&target.id, "first backup");

        // For testing, empty the target directory.
        let _ = std::fs::remove_dir_all(&targetdir);
        let _ = std::fs::create_dir(&targetdir);
        assert!(!testfile.exists());

        // Command
        let cmd = RestoreTarget;
        let payload = RestoreTargetPayload {
            target_id: target.id,
            backup_id: 1,
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_ok());
        assert!(testfile.exists());
    }

    #[test]
    fn it_returns_err_when_target_not_found() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Command
        let cmd = RestoreTarget;
        let payload = RestoreTargetPayload {
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
        let cmd = RestoreTarget;
        let payload = RestoreTargetPayload {
            target_id: target.id,
            backup_id: 1,
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_err());
    }
}

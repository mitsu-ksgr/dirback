//!
//! # BackupTarget command
//!
use crate::commands::Command;

use dirback::adapter::GetTargetAdapter;
use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::infra::service::targz_backup_service::TargzBackupService;
use dirback::usecase::backup::BackupUsecase;
use dirback::usecase::dto::Target;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BackupTargetPayload {
    pub target_id: String,
    pub note: String,
}

pub struct BackupTarget;

impl Command for BackupTarget {
    type Payload = BackupTargetPayload;
    type Output = Target;

    fn execute(
        &self,
        datadir: &std::path::Path,
        payload: Self::Payload,
    ) -> anyhow::Result<Self::Output> {
        let mut repo = FileStorageTargetRepository::new(datadir);
        let service = TargzBackupService::new();
        let mut usecase = BackupUsecase::new(&mut repo, &service);
        usecase.execute(&payload.target_id, &payload.note)?;

        let adapter = GetTargetAdapter::new(&repo);
        Ok(adapter.execute(&payload.target_id).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirback::infra::repository::file_storage::FileStorageTargetRepository;
    use dirback::internal::TargetRepository;

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Test target
        let mut repo = FileStorageTargetRepository::new(&basedir);
        let target = repo.add("TestTarget", std::path::Path::new(".")).unwrap();

        // Command
        let cmd = BackupTarget;
        let payload = BackupTargetPayload {
            target_id: target.id.clone(),
            note: String::from("Test backup!"),
        };

        assert_eq!(target.backups.len(), 0);

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_ok());

        let got = result.unwrap();
        assert_eq!(got.id, target.id);
        assert_eq!(got.name, target.name);
        assert_eq!(got.backups.len(), target.backups.len() + 1);

        let be = got.backups.first();
        assert!(be.is_some());
        let be = be.unwrap();
        assert_eq!(be.note, "Test backup!");
    }

    #[test]
    fn it_returns_err_if_target_not_found() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Command
        let cmd = BackupTarget;
        let payload = BackupTargetPayload {
            target_id: String::from("xxxxx-xxxxx-xxxxx"),
            note: String::from("Test backup!"),
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_err());
    }
}

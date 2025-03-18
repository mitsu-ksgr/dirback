//!
//! # Restore usecase
//!

use crate::domain::repository::targets::TargetRepository;
use crate::domain::service::backup_service::BackupService;

pub struct RestoreUsecase<'a, R: TargetRepository, B: BackupService> {
    repo: &'a mut R,
    backup_service: &'a B,
}

impl<'a, R: TargetRepository, B: BackupService> RestoreUsecase<'a, R, B> {
    pub fn new(repo: &'a mut R, backup_service: &'a B) -> Self {
        Self {
            repo,
            backup_service,
        }
    }

    pub fn execute(&mut self, target_id: &str, backup_id: u32) -> anyhow::Result<()> {
        let target = self
            .repo
            .load(target_id)
            .ok_or_else(|| anyhow::anyhow!("Target({}) not found.", target_id))?;

        let entry = target
            .find_backup_entry(backup_id)
            .ok_or_else(|| anyhow::anyhow!("BackupEntry({}) not found", backup_id))?;

        self.backup_service.restore(&entry.path, &target.path)
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::repository::in_memory::InMemoryTargetRepository;
    use crate::usecase::usecase_test_helper::*;
    use std::path::Path;

    #[test]
    fn it_works() {
        let mut repo = InMemoryTargetRepository::new();
        let (backup_service, _, restore_counter) = TestBackupService::new();

        let target_name = "Test target";
        let target_path = Path::new("test-target");

        let mut target = repo.add(target_name, target_path).unwrap();
        let target_id = target.id.clone();

        let bkpath = Path::new("test-backups");
        let entry = target.new_backup_entry(&bkpath, "tar.gz");
        let entry_id = entry.id;
        let _ = target.register_backup_entry(entry);
        let _ = repo.update(&target);

        {
            let mut restore = RestoreUsecase::new(&mut repo, &backup_service);
            let result = restore.execute(&target_id, entry_id);
            assert!(result.is_ok());
        }

        assert_eq!(
            *restore_counter.borrow(),
            1,
            "restore() should be called once."
        );
    }

    #[test]
    fn it_returns_err_if_non_exsisting_target_id() {
        let mut repo = InMemoryTargetRepository::new();
        let (backup_service, _, restore_counter) = TestBackupService::new();

        let target_name = "Test target";
        let target_path = Path::new("test-target");
        let _ = repo.add(target_name, target_path);

        {
            let mut restore = RestoreUsecase::new(&mut repo, &backup_service);
            let result = restore.execute("non-existing-id", 123);
            assert!(result.is_err());
        }

        assert_eq!(
            *restore_counter.borrow(),
            0,
            "restore() should not be called."
        );
    }

    #[test]
    fn it_returns_err_if_non_existing_backup_entry_id() {
        let mut repo = InMemoryTargetRepository::new();
        let (backup_service, _, restore_counter) = TestBackupService::new();

        let target_name = "Test target";
        let target_path = Path::new("test-target");

        let mut target = repo.add(target_name, target_path).unwrap();
        let target_id = target.id.clone();

        let bkpath = Path::new("test-backups");
        let entry = target.new_backup_entry(&bkpath, "tar.gz");
        let _ = target.register_backup_entry(entry.clone());
        let _ = repo.update(&target);

        {
            let invalid_id = entry.id + 1;
            let mut restore = RestoreUsecase::new(&mut repo, &backup_service);
            let result = restore.execute(&target_id, invalid_id);
            assert!(result.is_err());
        }

        assert_eq!(
            *restore_counter.borrow(),
            0,
            "restore() should not be called."
        );
    }
}

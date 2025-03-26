//!
//! # Backup usecase
//!

use crate::domain::repository::targets::TargetRepository;
use crate::domain::service::backup_service::BackupService;

pub struct BackupUsecase<'a, R: TargetRepository, B: BackupService> {
    repo: &'a mut R,
    backup_service: &'a B,
}

impl<'a, R: TargetRepository, B: BackupService> BackupUsecase<'a, R, B> {
    pub fn new(repo: &'a mut R, backup_service: &'a B) -> Self {
        Self {
            repo,
            backup_service,
        }
    }

    pub fn execute(&mut self, target_id: &str, note: &str) -> anyhow::Result<()> {
        let mut target = self.repo.load(target_id).unwrap();

        // Make path to the backup file
        let backup_path = self.repo.make_backup_dir_path(&target);

        // Make a backup entry
        let mut entry = target.new_backup_entry(&backup_path, "tar.gz");
        entry.note = note.to_string();

        // Backup
        self.backup_service.backup(&target.path, &entry.path)?;

        // Save the backup entry.
        #[allow(clippy::never_loop)]
        loop {
            if target.register_backup_entry(entry).is_err() {
                break;
            }
            if self.repo.update(&target).is_err() {
                break;
            }

            return Ok(());
        }

        // TODO: remove backup file??
        // The backup was created successfully,
        // but failed to save the backup entry.
        anyhow::bail!("Error: failed to save the backup entry.");
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
        let (backup_service, backup_counter, _) = TestBackupService::new();

        let target_name = "Test target";
        let target_path = Path::new("/tmp/path/to/target");

        let target = repo.add(target_name, target_path).unwrap();
        let target_id = target.id;
        let before_backup_count = target.backups.len();

        {
            let mut backup = BackupUsecase::new(&mut repo, &backup_service);
            let result = backup.execute(&target_id, "this is test backup");
            assert!(result.is_ok());
        }

        let target = repo.load(&target_id).unwrap();
        assert_eq!(
            target.backups.len(),
            before_backup_count + 1,
            "backup count should be increased by 1."
        );
        assert_eq!(
            *backup_counter.borrow(),
            1,
            "backup() should be called once."
        );
    }

    #[test]
    fn it_can_be_called_multiple_times() {
        let mut repo = InMemoryTargetRepository::new();
        let (backup_service, backup_counter, _) = TestBackupService::new();

        let target1 = repo
            .add("Target1", Path::new("/tmp/path/to/target1"))
            .unwrap();
        let target1_id = target1.id;
        let before_backup_count = target1.backups.len();

        let target2 = repo
            .add("Target2", Path::new("/tmp/path/to/target2"))
            .unwrap();
        let target2_id = target2.id;

        {
            let mut backup = BackupUsecase::new(&mut repo, &backup_service);
            let _ = backup.execute(&target1_id, "this is target1 backup");
            let _ = backup.execute(&target1_id, "this is target1 backup");
            let _ = backup.execute(&target2_id, "this is target2 backup");
        }

        let target1 = repo.load(&target1_id).unwrap();
        assert_eq!(
            target1.backups.len(),
            before_backup_count + 2,
            "backup count should be increased by 1."
        );
        assert_eq!(
            *backup_counter.borrow(),
            3,
            "backup() should be called 3 times."
        );
    }
}

//!
//! # Backup usecase
//!

use crate::domain::repository::targets::TargetRepository;
use crate::domain::service::backup_service::BackupService;
use crate::infra::app_path;

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
        let mut backup_path = app_path::data_dir().unwrap();
        let parts = ["targets", &target.id, "backups"];
        for part in &parts {
            backup_path.push(part);
        }

        // Make a backup entry
        let mut entry = target.new_backup_entry(&backup_path, "tar.gz");
        entry.note = note.to_string();

        // Backup
        self.backup_service.backup(&target.path, &entry.path);

        // Save target.
        // TODO: Error handling?
        let _ = target.register_backup_entry(entry)?;
        let _ = self.repo.update(&target);

        Ok(())
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::service::backup_service::BackupService;
    use crate::infra::repository::in_memory::InMemoryTargetRepository;
    use std::cell::RefCell;
    use std::path::Path;
    use std::rc::Rc;

    struct TestBackupService {
        backup_counter: Rc<RefCell<usize>>,
    }

    impl TestBackupService {
        fn new() -> (Self, Rc<RefCell<usize>>) {
            let counter = Rc::new(RefCell::new(0));
            (
                Self {
                    backup_counter: counter.clone(),
                },
                counter,
            )
        }
    }

    impl BackupService for TestBackupService {
        fn backup(&self, src: &Path, dest: &Path) -> anyhow::Result<()> {
            *self.backup_counter.borrow_mut() += 1;
            Ok(())
        }

        fn restore(&self, src: &Path, dest: &Path) -> anyhow::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn it_works() {
        let mut repo = InMemoryTargetRepository::new();
        let (backup_service, backup_counter) = TestBackupService::new();

        let target_name = "Test target";
        let target_path = Path::new("/tmp/path/to/target");

        let target = repo.add(target_name, target_path).unwrap();
        let target_id = target.id;
        let before_backup_count = target.backups.len();

        {
            let mut backup = BackupUsecase::new(&mut repo, &backup_service);
            let _ = backup.execute(&target_id, "this is test backup");
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
        let (backup_service, backup_counter) = TestBackupService::new();

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

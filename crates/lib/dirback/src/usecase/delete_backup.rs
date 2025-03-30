//!
//! # Delete backup usecase
//!

use crate::domain::repository::targets::TargetRepository;
use crate::usecase::dto::BackupEntry;

pub struct DeleteBackupUsecase<'a, R: TargetRepository> {
    repo: &'a mut R,
}

impl<'a, R: TargetRepository> DeleteBackupUsecase<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn execute(&mut self, target_id: &str, backup_id: u32) -> anyhow::Result<BackupEntry> {
        let entry = self.repo.delete_backup(target_id, backup_id)?;
        Ok(entry.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::repository::in_memory::InMemoryTargetRepository;
    use std::path::Path;

    #[test]
    fn it_works() {
        let mut repo = InMemoryTargetRepository::new();

        let mut target = repo.add("TestTarget", Path::new(".")).unwrap();
        for i in 1..=3 {
            let ts = crate::domain::model::timestamp::Timestamp::now();
            let note = format!("Test target backup {i}");
            let bk = crate::domain::model::BackupEntry::new(i, Path::new("."), ts, &note);
            target.backups.push(bk);
        }
        let target = repo.update(&target).unwrap();
        let before_backup_count = target.backups.len();

        let del_backup_id = 2;
        let mut usecase = DeleteBackupUsecase::new(&mut repo);
        let result = usecase.execute(&target.id, del_backup_id);
        assert!(result.is_ok());

        let entry = result.unwrap();
        assert_eq!(entry.id, del_backup_id);

        let target = repo.load(&target.id).unwrap();
        assert_eq!(target.backups.len(), before_backup_count - 1);
    }
}

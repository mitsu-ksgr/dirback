//!
//! # InMemory Repository
//!
//! This repository is for testing.
//! No data sharing between instances.
//!

use crate::domain::model::backup_entry::BackupEntry;
use crate::domain::model::target::Target;
use crate::domain::repository::targets::TargetRepository;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct InMemoryTargetRepository {
    targets: Vec<Target>,
}

impl InMemoryTargetRepository {
    pub fn new() -> Self {
        Self {
            targets: Vec::<Target>::new(),
        }
    }
}

impl TargetRepository for InMemoryTargetRepository {
    fn load_all(&self) -> anyhow::Result<Vec<Target>> {
        Ok(self.targets.clone())
    }

    fn load(&self, target_id: &str) -> Option<Target> {
        self.targets.iter().find(|t| t.id == target_id).cloned()
    }

    fn update(&mut self, target: &Target) -> anyhow::Result<Target> {
        self.targets
            .iter_mut()
            .find(|t| t.id == target.id)
            .map(|t| {
                *t = target.clone();
                target.clone()
            })
            .ok_or_else(|| anyhow::anyhow!("target not found"))
    }

    fn add(&mut self, name: &str, target_path: &Path) -> anyhow::Result<Target> {
        let new_id = uuid::Uuid::new_v4();
        let target = Target::new(&new_id.to_string(), name, target_path);
        self.targets.push(target.clone());
        Ok(target)
    }

    fn delete_backup(&mut self, target_id: &str, backup_id: u32) -> anyhow::Result<BackupEntry> {
        let mut target = self
            .load(target_id)
            .ok_or_else(|| anyhow::anyhow!("Target not found ('{}').", target_id))?;

        if let Some(pos) = target.backups.iter().position(|b| b.id == backup_id) {
            let entry = target.backups.remove(pos);
            let _ = self.update(&target)?;
            Ok(entry)
        } else {
            anyhow::bail!(
                "Target('{target_id}') does not have specified backup(id='{backup_id}')."
            );
        }
    }

    fn delete_target(&mut self, target_id: &str) -> anyhow::Result<Target> {
        if let Some(pos) = self.targets.iter().position(|t| t.id == target_id) {
            Ok(self.targets.remove(pos))
        } else {
            anyhow::bail!("Target not found ('{target_id}')")
        }
    }

    /// For testing.
    fn make_backup_dir_path(&self, _target: &Target) -> PathBuf {
        PathBuf::new()
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_new() {
        let repo = InMemoryTargetRepository::new();
        assert_eq!(repo.targets.len(), 0);
    }

    #[test]
    fn test_load_all() {
        let mut repo = InMemoryTargetRepository::new();
        assert_eq!(repo.targets.len(), 0);
        for i in 1..=10 {
            let name = format!("Test Target {i}");
            let path = PathBuf::from(format!("target{i}"));
            let _ = repo.add(&name, &path);
        }
        assert_eq!(repo.targets.len(), 10);

        let targets = repo.load_all().unwrap();
        assert_eq!(targets.len(), 10);
    }

    #[test]
    fn test_load() {
        let mut repo = InMemoryTargetRepository::new();
        let path = Path::new("target");
        let t1 = repo.add("Test Target", path).unwrap();
        let t2 = repo.load(&t1.id).unwrap();
        assert_eq!(t1.id, t2.id);
        assert_eq!(t1.path, t2.path);
    }

    mod test_update {
        use super::*;

        #[test]
        fn it_works() {
            let mut repo = InMemoryTargetRepository::new();

            let path = Path::new("target");
            let mut target = repo.add("Test Target", path).unwrap();
            target.path.push("foo");

            let id = target.id.to_string();
            let result = repo.update(&target);
            assert!(result.is_ok());

            let expect = Path::new("target").join("foo");

            let result = result.unwrap();
            assert_eq!(result.path, expect);

            let result = repo.load(&id).unwrap();
            assert_eq!(result.path, expect);
        }

        #[test]
        fn it_returns_err_when_id_does_not_exists() {
            let mut repo = InMemoryTargetRepository::new();

            let path = Path::new("target");
            let target = repo.add("Test Target", path).unwrap();

            let mut target2 = target.clone();
            target2.id = String::from("nonexistent-id");

            let result = repo.update(&target2);
            assert!(result.is_err());
        }

        #[test]
        fn it_work_when_update_backups() {
            let mut repo = InMemoryTargetRepository::new();

            let make_backup_path = |target_id: &str| -> std::path::PathBuf {
                ["dirback", target_id, "backups"].iter().collect()
            };

            // Add test targets.
            for i in 1..=3 {
                let name = format!("target{i}");
                let path = PathBuf::from(format!("path-to-{name}"));
                let mut target = repo.add(&name, &path).unwrap();

                // Add backups.
                let backup_dir = make_backup_path(&target.id);
                for _k in 1..=(2 + i) {
                    let entry = target.new_backup_entry(&backup_dir, "tar.gz");
                    let _ = target.register_backup_entry(entry);
                }

                let _ = repo.update(&target);
            }

            // Update test.
            let id = repo.targets[1].id.to_string(); // test target.
            let mut target = repo.load(&id).unwrap();
            let before_update = target.clone();

            let backup_dir = make_backup_path(&target.id);
            let entry = target.new_backup_entry(&backup_dir, "tar.gz");
            let _ = target.register_backup_entry(entry);
            let _ = repo.update(&target);

            let target = repo.load(&id).unwrap();
            assert_ne!(target.backups.len(), before_update.backups.len());
        }
    }

    #[test]
    fn test_add() {
        let mut repo = InMemoryTargetRepository::new();
        let path = Path::new("target");

        assert_eq!(repo.targets.len(), 0);
        let target = repo.add("Test Target", path).unwrap();

        assert_eq!(repo.targets.len(), 1);
        assert_eq!(target.path, path);
        assert_eq!(target.backups.len(), 0);
    }

    mod delete_backup {
        use super::*;

        #[test]
        fn it_works() {
            let mut repo = InMemoryTargetRepository::new();
            let mut target = repo.add("TestTarget", Path::new(".")).unwrap();
            for i in 1..=3 {
                let ts = crate::domain::model::timestamp::Timestamp::now();
                let note = format!("Test target backup {i}");
                let bk = BackupEntry::new(i, Path::new("."), ts, &note);
                target.backups.push(bk);
            }
            let target = repo.update(&target).unwrap();

            let before_backup_count = target.backups.len();

            let del_id = 2;
            let result = repo.delete_backup(&target.id, del_id);
            assert!(result.is_ok());

            let del_bk = result.unwrap();
            assert_eq!(del_bk.id, del_id);

            let target = repo.load(&target.id).unwrap();
            assert_eq!(target.backups.len(), before_backup_count - 1);
            assert!(
                target.backups.iter().all(|b| b.id != del_bk.id),
                "Deleted backup entry should not be in the repository."
            );
        }

        #[test]
        fn it_returns_err_when_non_existent_target_id() {
            let mut repo = InMemoryTargetRepository::new();
            let result = repo.delete_backup("non-exists-target-id", 1);
            assert!(result.is_err());
        }

        #[test]
        fn it_returns_err_when_non_existent_backup_id() {
            let mut repo = InMemoryTargetRepository::new();
            let mut target = repo.add("TestTarget", Path::new(".")).unwrap();
            for i in 1..=3 {
                let ts = crate::domain::model::timestamp::Timestamp::now();
                let note = format!("Test target backup {i}");
                let bk = BackupEntry::new(i, Path::new("."), ts, &note);
                target.backups.push(bk);
            }
            let target = repo.update(&target).unwrap();

            let before_backup_count = target.backups.len();

            let result = repo.delete_backup(&target.id, 123);
            assert!(result.is_err());

            let target = repo.load(&target.id).unwrap();
            assert_eq!(target.backups.len(), before_backup_count);
        }
    }

    mod delete_target {
        use super::*;

        #[test]
        fn it_works() {
            let mut repo = InMemoryTargetRepository::new();
            let mut ids = Vec::<String>::new();
            for i in 1..=3 {
                let name = format!("Test Target {i}");
                let path = PathBuf::from(format!("target{i}"));
                let target = repo.add(&name, &path).unwrap();
                ids.push(target.id);
            }

            let before_target_count = repo.load_all().unwrap().len();

            let del_target_id = ids[1].clone();
            let result = repo.delete_target(&del_target_id);
            assert!(result.is_ok());

            let target = result.unwrap();
            assert_eq!(target.id, del_target_id);

            let targets = repo.load_all().unwrap();
            assert_eq!(targets.len(), before_target_count - 1);

            assert!(
                targets.iter().all(|t| t.id != del_target_id),
                "Deleted target should not be in the repository."
            );
        }

        #[test]
        fn it_returns_err_when_non_existent_target_id() {
            let mut repo = InMemoryTargetRepository::new();
            let mut ids = Vec::<String>::new();
            for i in 1..=3 {
                let name = format!("Test Target {i}");
                let path = PathBuf::from(format!("target{i}"));
                let target = repo.add(&name, &path).unwrap();
                ids.push(target.id);
            }

            let before_target_count = repo.load_all().unwrap().len();

            let result = repo.delete_target("non-exists-target-id");
            assert!(result.is_err());

            let targets = repo.load_all().unwrap();
            assert_eq!(targets.len(), before_target_count);
        }
    }
}

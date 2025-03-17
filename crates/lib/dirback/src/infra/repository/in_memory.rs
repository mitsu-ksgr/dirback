//!
//! # InMemory Repository
//!
//! This repository is for testing.
//! No data sharing between instances.
//!

use crate::domain::model::target::Target;
use crate::domain::repository::targets::TargetRepository;
use std::path::{Path, PathBuf};

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
        let target = Target::new(&new_id.to_string(), name, &target_path);
        self.targets.push(target.clone());
        Ok(target)
    }

    /// For testing.
    fn make_backup_dir_path(&self, target: &Target) -> PathBuf {
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
            let path = PathBuf::from(format!("/tmp/dirback/target{i}"));
            let _ = repo.add(&name, &path);
        }
        assert_eq!(repo.targets.len(), 10);

        let targets = repo.load_all().unwrap();
        assert_eq!(targets.len(), 10);
    }

    #[test]
    fn test_load() {
        let mut repo = InMemoryTargetRepository::new();
        let path = Path::new("/tmp/dirback/target");
        let t1 = repo.add("Test Target", &path).unwrap();
        let t2 = repo.load(&t1.id).unwrap();
        assert_eq!(t1.id, t2.id);
        assert_eq!(t1.path, t2.path);
    }

    mod test_update {
        use super::*;

        #[test]
        fn it_works() {
            let mut repo = InMemoryTargetRepository::new();

            let path = Path::new("/tmp/dirback/target");
            let mut target = repo.add("Test Target", &path).unwrap();
            target.path.push("foo");

            let id = target.id.to_string();
            let result = repo.update(&target);
            assert!(result.is_ok());

            let result = result.unwrap();
            assert_eq!(result.path.to_string_lossy(), "/tmp/dirback/target/foo");

            let result = repo.load(&id).unwrap();
            assert_eq!(result.path.to_string_lossy(), "/tmp/dirback/target/foo");
        }

        #[test]
        fn it_returns_err_when_id_does_not_exists() {
            let mut repo = InMemoryTargetRepository::new();

            let path = Path::new("/tmp/dirback/target");
            let target = repo.add("Test Target", &path).unwrap();

            let mut target2 = target.clone();
            target2.id = String::from("nonexistent-id");

            let result = repo.update(&target2);
            assert!(result.is_err());
        }

        #[test]
        fn it_works2() {
            let mut repo = InMemoryTargetRepository::new();

            // Add test targets.
            for i in 1..=3 {
                let name = format!("target{i}");
                let path = PathBuf::from(format!("/tmp/{name}"));
                let mut target = repo.add(&name, &path).unwrap();

                // Add backups.
                let backup_dir =
                    PathBuf::from(&format!("/tmp/dirback/targets/{}/backups", target.id));
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

            let backup_dir = PathBuf::from(&format!("/tmp/dirback/targets/{}/backups", target.id));
            let entry = target.new_backup_entry(&backup_dir, "tar.gz");
            let _ = target.register_backup_entry(entry);
            repo.update(&target);

            let target = repo.load(&id).unwrap();
            assert_ne!(target.backups.len(), before_update.backups.len());
        }
    }

    #[test]
    fn test_add() {
        let mut repo = InMemoryTargetRepository::new();
        let path = Path::new("/tmp/dirback/target");

        assert_eq!(repo.targets.len(), 0);
        let target = repo.add("Test Target", &path).unwrap();

        assert_eq!(repo.targets.len(), 1);
        assert_eq!(target.path, path);
        assert_eq!(target.backups.len(), 0);
    }
}

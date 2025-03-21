//!
//! # FileStorage Repository
//!
//! Store target informations to file storage.
//!
//!
//! ## Direcotry structure
//!
//! ```ascii
//! {base_dir}/
//! └─ targets/
//!    └─ {target_id}/
//!       ├─ info.json
//!       └─ backups/
//!          └─ {backup_id}_{backup_timestamp}.tar.gz
//! ```
//!

use crate::domain::model::target::Target;
use crate::domain::repository::targets::TargetRepository;
use std::path::{Path, PathBuf};

//-----------------------------------------------------------------------------
// Helper
//-----------------------------------------------------------------------------
const TARGET_INFO_DIR_NAME: &str = "targets";
const TARGET_INFO_FILE_NAME: &str = "info.json";
const BACKUP_DIR_NAME: &str = "backups";

fn create_target_info_dir_path(base_dir: &Path, target_id: Option<&str>) -> PathBuf {
    let path = base_dir.join(TARGET_INFO_DIR_NAME);

    match target_id {
        Some(target_id) => path.join(target_id),
        None => path,
    }
}

fn create_target_info_file_path(base_dir: &Path, target_id: &str) -> PathBuf {
    let path = create_target_info_dir_path(base_dir, Some(target_id));
    path.join(TARGET_INFO_FILE_NAME)
}

fn create_backup_dir_path(base_dir: &Path, target_id: &str) -> PathBuf {
    let path = create_target_info_dir_path(base_dir, Some(target_id));
    path.join(BACKUP_DIR_NAME)
}

//-----------------------------------------------------------------------------
// FileStorageTargetRepository
//-----------------------------------------------------------------------------
pub struct FileStorageTargetRepository {
    base_dir: PathBuf,
}

impl FileStorageTargetRepository {
    /// Create a new FileStorageTargetRepository instance.
    ///
    /// - base_dir ... The base path for the application data directory.
    pub fn new(base_dir: &Path) -> Self {
        let repo = Self {
            base_dir: base_dir.to_path_buf(),
        };

        repo.ensure_directory_structure();
        repo
    }

    /// Ensure directory structure.
    fn ensure_directory_structure(&self) {
        let targets_dir = create_target_info_dir_path(&self.base_dir, None);
        if !targets_dir.exists() {
            let _ = std::fs::create_dir_all(&targets_dir);
        }
    }
}

impl TargetRepository for FileStorageTargetRepository {
    fn load_all(&self) -> anyhow::Result<Vec<Target>> {
        let mut targets = Vec::new();

        let dir_path = create_target_info_dir_path(&self.base_dir, None);
        for dir in std::fs::read_dir(&dir_path)? {
            if dir.is_err() {
                continue;
            }

            let dir = dir.unwrap().path();
            let info_file_path = dir.join(TARGET_INFO_FILE_NAME);
            let target: Target = jsonfile::read(&info_file_path)?;
            targets.push(target);
        }

        Ok(targets)
    }

    fn load(&self, target_id: &str) -> Option<Target> {
        let info_path = create_target_info_file_path(&self.base_dir, target_id);
        jsonfile::read(&info_path).unwrap_or_default()
    }

    fn update(&mut self, target: &Target) -> anyhow::Result<Target> {
        let info_path = create_target_info_file_path(&self.base_dir, &target.id);
        jsonfile::write(&info_path, &target)?;
        Ok(target.clone())
    }

    fn add(&mut self, name: &str, target_path: &Path) -> anyhow::Result<Target> {
        // 1. make a new target
        let new_id = uuid::Uuid::new_v4();
        let target = Target::new(&new_id.to_string(), name, target_path);

        // 2. make directory for target-info.
        let dir = create_target_info_dir_path(&self.base_dir, Some(&target.id));
        std::fs::create_dir_all(&dir)?;

        // 3. create a target-info file.
        let info_path = dir.join(TARGET_INFO_FILE_NAME);
        jsonfile::write(&info_path, &target)?;

        // 4. make directory for backups of targets.
        let bk_dir = dir.join(BACKUP_DIR_NAME);
        std::fs::create_dir_all(&bk_dir)?;

        // 5. return new target
        Ok(target)
    }

    fn make_backup_dir_path(&self, target: &Target) -> PathBuf {
        create_backup_dir_path(&self.base_dir, &target.id)
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    mod test_create_target_info_dir_path {
        use super::*;

        #[test]
        fn withtou_target() {
            let base = Path::new("tmp");
            let info_path = create_target_info_dir_path(&base, None);
            assert_eq!(info_path, base.join(TARGET_INFO_DIR_NAME));
        }

        #[test]
        fn with_target() {
            let base = Path::new("tmp");
            let target = Target::new("abcd123", "Test target", Path::new("path"));
            let expect = base.join(TARGET_INFO_DIR_NAME).join("abcd123");

            let result = create_target_info_dir_path(&base, Some(&target.id));
            assert_eq!(result, expect);
        }
    }

    mod test_create_target_info_file_path {
        use super::*;

        #[test]
        fn it_works() {
            let base = Path::new("tmp");
            let id = "xxxx-xxxx";

            let expect = ["tmp", TARGET_INFO_DIR_NAME, id, TARGET_INFO_FILE_NAME];
            let expect: PathBuf = expect.iter().collect();

            let result = create_target_info_file_path(&base, &id);
            assert_eq!(result, expect);
        }
    }

    mod test_create_backup_dir_path {
        use super::*;

        #[test]
        fn it_works() {
            let base = Path::new("tmp");
            let id = "xxxx-xxxx";

            let expect = ["tmp", TARGET_INFO_DIR_NAME, id, BACKUP_DIR_NAME];
            let expect: PathBuf = expect.iter().collect();

            let result = create_backup_dir_path(&base, &id);
            assert_eq!(result, expect);
        }
    }

    #[test]
    fn test_new_with_ensure_directory_structure() {
        let temp = mktemp::TempDir::new().unwrap();
        let repo = FileStorageTargetRepository::new(&temp.path());

        assert_eq!(repo.base_dir, temp.path());
        assert!(
            temp.path().join(TARGET_INFO_DIR_NAME).exists(),
            "it should create a targets directory in the base_dir directory."
        );
    }

    mod add {
        use super::*;

        // TODO
        #[test]
        fn it_add_new_target() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());

            let test_target: PathBuf = ["path", "to", "target"].iter().collect();

            let result = repo.add("Test Target", &test_target);
            assert!(result.is_ok());
        }

        #[test]
        fn it_create_target_info_file() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());

            let test_target: PathBuf = ["path", "to", "target"].iter().collect();
            let target = repo.add("Test Target", &test_target).unwrap();

            let info_dir_path = create_target_info_dir_path(&temp.path(), Some(&target.id));
            assert!(
                info_dir_path.exists(),
                "it should create a directory for target info."
            );

            let info_file_path = info_dir_path.join(TARGET_INFO_FILE_NAME);
            assert!(
                info_file_path.exists(),
                "it should create a target info file."
            );

            let bk_path = info_dir_path.join(BACKUP_DIR_NAME);
            assert!(bk_path.exists(), "it should create a backup directory.");

            let info: Target = jsonfile::read(&info_file_path).unwrap();
            assert_eq!(target, info);
        }
    }

    mod load_all {
        use super::*;

        #[test]
        fn it_returns_empty_vector_if_taget_is_not_yet_registered() {
            let temp = mktemp::TempDir::new().unwrap();
            let repo = FileStorageTargetRepository::new(&temp.path());

            let result = repo.load_all();
            assert!(result.is_ok());
            assert!(result.unwrap().is_empty());
        }

        #[test]
        fn it_returns_vector_contain_all_targets() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());

            // Make test targets.
            let mut targets = Vec::new();
            for i in 1..=3 {
                let name = format!("tagret{i}");
                let path: PathBuf = ["path", "to", &name].iter().collect();
                targets.push(repo.add(&name, &path).unwrap());
            }

            // Test
            let result = repo.load_all();
            assert!(result.is_ok());

            let mut result = result.unwrap();
            assert_eq!(result.len(), targets.len());

            result.sort_by(|a, b| a.name.cmp(&b.name));
            assert_eq!(result, targets);
        }
    }

    mod load {
        use super::*;

        #[test]
        fn it_works() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());
            let target = repo.add("TestTarget", Path::new("target")).unwrap();

            let result = repo.load(&target.id);
            assert!(result.is_some());
            assert_eq!(target, result.unwrap());
        }

        #[test]
        fn it_returns_none_if_target_is_not_exists() {
            let temp = mktemp::TempDir::new().unwrap();
            let repo = FileStorageTargetRepository::new(&temp.path());

            let result = repo.load("nonexistent-id");
            assert!(result.is_none());
        }
    }

    mod update {
        use super::*;

        #[test]
        fn it_works() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());

            // test: backup
            let target1 = repo.add("TestTarget", Path::new("tmp")).unwrap();
            let mut target2 = target1.clone();
            let bk_path = repo.make_backup_dir_path(&target2);
            let entry = target2.new_backup_entry(&bk_path, "tar.gz");
            let _ = target2.register_backup_entry(entry);

            let result = repo.update(&target2);
            assert!(result.is_ok());

            let result = result.unwrap();
            assert_ne!(result, target1);
            assert_eq!(result, target2);
        }

        #[test]
        fn it_returns_err_if_target_is_not_exists() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());

            let target = Target::new("xxx", "Fake target", Path::new("tmp"));

            let result = repo.update(&target);
            assert!(result.is_err());
        }
    }

    mod make_backup_dir_path {
        use super::*;

        #[test]
        fn it_works() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());
            let target = repo.add("TestTarget", Path::new("target")).unwrap();

            let mut expect = temp.path();
            for part in [TARGET_INFO_DIR_NAME, &target.id, BACKUP_DIR_NAME] {
                expect.push(part);
            }

            let result = repo.make_backup_dir_path(&target);
            assert_eq!(result, expect);
        }
    }
}

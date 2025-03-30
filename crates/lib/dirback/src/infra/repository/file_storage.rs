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

use crate::domain::model::backup_entry::BackupEntry;
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

    fn delete_backup(&mut self, target_id: &str, backup_id: u32) -> anyhow::Result<BackupEntry> {
        let mut target = self
            .load(target_id)
            .ok_or_else(|| anyhow::anyhow!("Target not found ('{target_id}')."))?;

        if let Some(pos) = target.backups.iter().position(|b| b.id == backup_id) {
            let entry = target.backups.remove(pos);
            let _ = self.update(&target)?;
            std::fs::remove_file(&entry.path)?;
            Ok(entry)
        } else {
            anyhow::bail!(
                "Target('{target_id}') does not have specified backup(id='{backup_id}')."
            );
        }
    }

    fn delete_target(&mut self, target_id: &str) -> anyhow::Result<Target> {
        if let Some(target) = self.load(target_id) {
            let dir = create_target_info_dir_path(&self.base_dir, Some(&target.id));
            let _ = std::fs::remove_dir_all(&dir);
            Ok(target)
        } else {
            anyhow::bail!("Target not found ('{target_id}').");
        }
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

    mod delete_backup {
        use super::*;

        #[test]
        fn it_works() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());

            let mut target = repo.add("TestTarget", Path::new(".")).unwrap();
            let bkdir = repo.make_backup_dir_path(&target);
            let mut entries = Vec::new();
            for i in 1..=3 {
                let ts = crate::domain::model::timestamp::Timestamp::now();
                let bk_path = bkdir.join(format!("{:0>3}_{}.test", i, ts.fmt()));
                let note = format!("TestTarget's backup {i}.");
                let entry = BackupEntry::new(i, &bk_path, ts, &note);
                target.backups.push(entry.clone());
                entries.push(entry);

                // Create test backup file.
                let _ = std::fs::File::create(&bk_path);
            }

            let target = repo.update(&target).unwrap();
            let before_backup_count = target.backups.len();

            let del_entry = &entries[1];
            let result = repo.delete_backup(&target.id, del_entry.id);
            if let Err(ref e) = result {
                println!("ERR: {:?}", e);
            }
            assert!(result.is_ok());

            let entry = result.unwrap();
            assert_eq!(entry.id, del_entry.id);
            assert!(
                !del_entry.path.exists(),
                "The backup file should be deleted."
            );

            let target = repo.load(&target.id).unwrap();
            assert_eq!(target.backups.len(), before_backup_count - 1);
            assert!(
                target.backups.iter().all(|b| b.id != entry.id),
                "Deleted backup entry should not be in the repository."
            );
        }

        #[test]
        fn it_returns_err_when_non_existent_target_id() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());
            let result = repo.delete_backup("non-exists-target-id", 1);
            assert!(result.is_err());
        }

        #[test]
        fn it_returns_err_when_existent_backup_id() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());

            let mut target = repo.add("TestTarget", Path::new(".")).unwrap();
            let bkdir = repo.make_backup_dir_path(&target);
            let mut entries = Vec::new();
            for i in 1..=3 {
                let ts = crate::domain::model::timestamp::Timestamp::now();
                let bk_path = bkdir.join(format!("{:0>3}_{}.test", i, ts.fmt()));
                let note = format!("TestTarget's backup {i}.");
                let entry = BackupEntry::new(i, &bk_path, ts, &note);
                target.backups.push(entry.clone());
                entries.push(entry);

                // Create test backup file.
                let _ = std::fs::File::create(&bk_path);
            }
            let target = repo.update(&target).unwrap();

            let before_backup_count = target.backups.len();

            let result = repo.delete_backup(&target.id, 123);
            if let Err(ref e) = result {
                println!("ERR: {:?}", e);
            }
            assert!(result.is_err());

            let target = repo.load(&target.id).unwrap();
            assert_eq!(target.backups.len(), before_backup_count);
        }
    }

    mod delete_target {
        use super::*;

        #[test]
        fn it_works() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());

            let mut ids = Vec::new();
            for i in 1..=3 {
                let name = format!("Test Target {i}");
                let target = repo.add(&name, Path::new(".")).unwrap();
                ids.push(target.id);
            }

            let before_target_count = repo.load_all().unwrap().len();

            let del_target_id = ids[1].clone();
            let del_target_info_path = temp.path().join("targets").join(&del_target_id);
            assert!(del_target_info_path.exists());

            let result = repo.delete_target(&del_target_id);
            assert!(result.is_ok());

            let target = result.unwrap();
            assert_eq!(target.id, del_target_id);
            assert!(target.path.exists(), "target.path should not be deleted!!!");

            let targets = repo.load_all().unwrap();
            assert_eq!(targets.len(), before_target_count - 1);
            assert!(
                targets.iter().all(|t| t.id != del_target_id),
                "Deleted target should not be in the repository."
            );
            assert!(
                !del_target_info_path.exists(),
                "Deleted target info directory should be deleted."
            );
        }

        #[test]
        fn it_returns_err_when_non_existent_target_id() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut repo = FileStorageTargetRepository::new(&temp.path());

            let mut ids = Vec::new();
            for i in 1..=3 {
                let name = format!("Test Target {i}");
                let target = repo.add(&name, Path::new(".")).unwrap();
                ids.push(target.id);
            }

            let before_target_count = repo.load_all().unwrap().len();

            let result = repo.delete_target("non-exists-target-id");
            assert!(result.is_err());

            let targets = repo.load_all().unwrap();
            assert_eq!(targets.len(), before_target_count);
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

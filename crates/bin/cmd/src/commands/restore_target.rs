//!
//! # RestoreTarget command
//!

use anyhow::Context;
use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::infra::service::targz_backup_service::TargzBackupService;
use dirback::usecase::restore::RestoreUsecase;

pub struct RestoreTarget;

impl cmd::Command for RestoreTarget {
    fn execute(&self, params: &cmd::CmdParams) -> anyhow::Result<()> {
        if params.args.len() < 2 {
            anyhow::bail!("Missing args: <target-id> <backup-id>");
        }

        let target_id = params.args[0].to_string();
        let backup_id = params.args[1].to_string();

        let backup_id = backup_id
            .parse::<u32>()
            .context(format!("Invalid Backup ID ('{backup_id}')."))?;

        println!("Target ID = {target_id}");
        println!("Backup ID = {backup_id}");

        let mut repo = FileStorageTargetRepository::new(&params.basedir);
        let service = TargzBackupService::new();

        let mut usecase = RestoreUsecase::new(&mut repo, &service);
        usecase.execute(&target_id, backup_id)?;

        println!("Restore completed!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cmd::*;
    use dirback::infra::service::targz_backup_service::TargzBackupService;
    use dirback::internal::TargetRepository;
    use dirback::usecase::backup::BackupUsecase;
    use std::io::{Read, Write};

    fn prepare_test_files(temp: &mktemp::TempDir) -> std::path::PathBuf {
        let target_path = temp.path().join("testproj");
        let _ = std::fs::create_dir_all(&target_path);

        let dir = target_path.clone();
        let _ = std::fs::File::create(&dir.join("foo.txt"));

        let dir = dir.join("foo");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::File::create(&dir.join("bar.txt"));
        let _ = std::fs::File::create(&dir.join(".hiddenfile"));

        let dir = dir.join("bar");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::File::create(&dir.join("baz.txt"));

        target_path
    }

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();

        // Create dirback directory.
        let base_path = temp.path().join("dirback");
        let _ = std::fs::create_dir_all(&base_path);

        // Create test files.
        let target_path = prepare_test_files(&temp);

        // Repo
        let mut repo = FileStorageTargetRepository::new(&base_path);

        // Create test target.
        let target = repo.add("TestTarget", &target_path).unwrap();

        // Backup
        let bk_service = TargzBackupService::new();
        let mut bk_usecase = BackupUsecase::new(&mut repo, &bk_service);
        let _ = bk_usecase.execute(&target.id, "first backup");

        // For testing, empty the target directory.
        let _ = std::fs::remove_dir_all(&target_path);
        let _ = std::fs::create_dir(&target_path);
        assert!(!target_path.join("foo.txt").exists());

        // CmdParams for Restore
        let args: Vec<String> = ["test", "restore", &target.id, "1"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &base_path).unwrap();

        // Restore
        let cmd = RestoreTarget {};
        let result = cmd.execute(&params);
        if let Err(ref e) = result {
            println!("{:?}", e);
        }
        assert!(result.is_ok());

        // Check file restored.
        assert!(target_path.join("foo.txt").exists());

        let dir = target_path.join("foo");
        assert!(dir.exists());
        assert!(dir.join("bar.txt").exists());
        assert!(dir.join(".hiddenfile").exists());

        let dir = dir.join("bar");
        assert!(dir.exists());
        assert!(dir.join("baz.txt").exists());
    }

    #[test]
    fn it_overwrite_existing_files() {
        let temp = mktemp::TempDir::new().unwrap();

        // Create dirback directory.
        let base_path = temp.path().join("dirback");
        let _ = std::fs::create_dir_all(&base_path);

        // Create test files.
        let target_path = prepare_test_files(&temp);
        let mod_file_path = target_path.join("foo.txt");
        let mut file = std::fs::File::create(&mod_file_path).unwrap();
        let _ = write!(file, "this is test");
        drop(file);

        // Repo
        let mut repo = FileStorageTargetRepository::new(&base_path);

        // Create test target.
        let target = repo.add("TestTarget", &target_path).unwrap();

        // Backup
        let bk_service = TargzBackupService::new();
        let mut bk_usecase = BackupUsecase::new(&mut repo, &bk_service);
        let _ = bk_usecase.execute(&target.id, "first backup");

        // For testing, update foo.txt file.
        let mut file = std::fs::File::create(&mod_file_path).unwrap();
        let _ = write!(file, "modified");
        drop(file);

        // CmdParams for Restore
        let args: Vec<String> = ["test", "restore", &target.id, "1"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &base_path).unwrap();

        // Restore
        let cmd = RestoreTarget {};
        let result = cmd.execute(&params);
        if let Err(ref e) = result {
            println!("{:?}", e);
        }
        assert!(result.is_ok());

        assert!(mod_file_path.exists());
        let mut file = std::fs::File::open(&mod_file_path).unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        assert_eq!(
            contents, "this is test",
            "The file contents should be restored."
        );
    }

    #[test]
    fn it_does_nothing_for_the_new_file() {
        let temp = mktemp::TempDir::new().unwrap();

        // Create dirback directory.
        let base_path = temp.path().join("dirback");
        let _ = std::fs::create_dir_all(&base_path);

        // Create test files.
        let target_path = prepare_test_files(&temp);

        // Repo
        let mut repo = FileStorageTargetRepository::new(&base_path);

        // Create test target.
        let target = repo.add("TestTarget", &target_path).unwrap();

        // Backup
        let bk_service = TargzBackupService::new();
        let mut bk_usecase = BackupUsecase::new(&mut repo, &bk_service);
        let _ = bk_usecase.execute(&target.id, "first backup");

        // For testing, create the new file.
        let new_file_path = target_path.join("fresh.file");
        let mut file = std::fs::File::create(&new_file_path).unwrap();
        let _ = write!(file, "super fresh!");
        drop(file);

        // CmdParams for Restore
        let args: Vec<String> = ["test", "restore", &target.id, "1"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &base_path).unwrap();

        // Restore
        let cmd = RestoreTarget {};
        let result = cmd.execute(&params);
        if let Err(ref e) = result {
            println!("{:?}", e);
        }
        assert!(result.is_ok());

        assert!(new_file_path.exists());
        let mut file = std::fs::File::open(&new_file_path).unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        assert_eq!(
            contents, "super fresh!",
            "The file contents should not be modified."
        );
    }

    #[test]
    fn it_returns_err_if_missing_args() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        {
            // Without the target-id.
            let args: Vec<String> = ["test", "restore"].iter().map(|s| s.to_string()).collect();
            let params = CmdParams::build(&args, &basedir).unwrap();

            let cmd = RestoreTarget {};
            let result = cmd.execute(&params);
            assert!(result.is_err(), "it should be fail without target-id.");
        }

        {
            // Without the backup-id.
            let args: Vec<String> = ["test", "restore", "target-id"]
                .iter()
                .map(|s| s.to_string())
                .collect();
            let params = CmdParams::build(&args, &basedir).unwrap();

            let cmd = RestoreTarget {};
            let result = cmd.execute(&params);
            assert!(result.is_err(), "it should be fail without backup-id.");
        }
    }

    #[test]
    fn it_returns_err_if_invalid_backup_id_passed() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        let args: Vec<String> = ["test", "restore", "non-existing-id", "invalid-backup-id"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &basedir).unwrap();

        let cmd = RestoreTarget {};
        let result = cmd.execute(&params);
        assert!(result.is_err());
    }
}

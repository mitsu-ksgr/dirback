//!
//! # BackupTarget command
//!

use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::infra::service::targz_backup_service::TargzBackupService;
use dirback::usecase::backup::BackupUsecase;

pub struct BackupTarget;

impl cmd::Command for BackupTarget {
    fn execute(&self, params: &cmd::CmdParams) -> anyhow::Result<()> {
        if params.args.is_empty() {
            anyhow::bail!("Missing args: <target-id> [note]");
        }

        let target_id = params.args[0].to_string();
        let note = params.args[1..].join(" ");

        let mut repo = FileStorageTargetRepository::new(&params.basedir);
        let service = TargzBackupService::new();

        let mut usecase = BackupUsecase::new(&mut repo, &service);
        usecase.execute(&target_id, &note)?;

        println!("Target({}) backup is complete.", target_id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cmd::*;
    use dirback::infra::repository::file_storage::FileStorageTargetRepository;
    use dirback::internal::TargetRepository;

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Test target
        let mut repo = FileStorageTargetRepository::new(&basedir);
        let target_path = std::fs::canonicalize(".").unwrap();
        let target = repo.add("TestTarget", &target_path).unwrap();

        // CmdParams
        let args: Vec<String> = ["test", "backup", &target.id, "backup testing"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &basedir).unwrap();

        // Execute
        let cmd = BackupTarget {};
        let result = cmd.execute(&params);
        if let Err(ref e) = result {
            println!("{:?}", e);
        }
        assert!(result.is_ok());

        // Check target and backupentry
        let t = repo.load(&target.id).unwrap();
        assert_eq!(t.backups.len(), target.backups.len() + 1);

        let bk = &t.backups[0];
        assert_eq!(bk.id, 1);
        assert_eq!(bk.note, "backup testing");

        // Check that the backup file was created.
        let bkpath = repo.make_backup_dir_path(&target);
        let targz_files: Vec<_> = std::fs::read_dir(&bkpath)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().is_file() && entry.file_name().to_string_lossy().ends_with(".tar.gz")
            })
            .collect();
        assert_eq!(targz_files.len(), 1, "tar.gz file should be created.");
    }

    #[test]
    fn it_fails_without_arguments() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        let args = vec![String::from("test"), String::from("backup")];
        let params = CmdParams::build(&args, &basedir).unwrap();

        let cmd = BackupTarget {};
        let result = cmd.execute(&params);
        assert!(result.is_err());
    }

    #[test]
    fn it_works_without_note() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Test target
        let mut repo = FileStorageTargetRepository::new(&basedir);
        let target_path = std::fs::canonicalize(".").unwrap();
        let target = repo.add("TestTarget", &target_path).unwrap();

        // CmdParams
        let args: Vec<String> = ["test", "backup", &target.id]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &basedir).unwrap();

        // Execute
        let cmd = BackupTarget {};
        let result = cmd.execute(&params);
        if let Err(ref e) = result {
            println!("{:?}", e);
        }
        assert!(result.is_ok());

        // Check target and backupentry
        let t = repo.load(&target.id).unwrap();
        assert_eq!(t.backups.len(), target.backups.len() + 1);

        let bk = &t.backups[0];
        assert_eq!(bk.id, 1);
        assert_eq!(bk.note, "");

        // Check that the backup file was created.
        let bkpath = repo.make_backup_dir_path(&target);
        let targz_files: Vec<_> = std::fs::read_dir(&bkpath)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().is_file() && entry.file_name().to_string_lossy().ends_with(".tar.gz")
            })
            .collect();
        assert_eq!(targz_files.len(), 1, "tar.gz file should be created.");
    }
}

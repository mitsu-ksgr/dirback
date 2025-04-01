//!
//! # DeleteBackup command
//!

use anyhow::Context;
use dirback::adapter::ShowTargetAdapter;
use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::usecase::delete_backup::DeleteBackupUsecase;

use std::io::BufRead;
use std::io::Write;

pub struct DeleteBackup {
    create_reader: fn() -> Box<dyn std::io::Read>,
}

impl DeleteBackup {
    pub fn new() -> Self {
        Self {
            create_reader: || Box::new(std::io::stdin()),
        }
    }
}

impl cmd::Command for DeleteBackup {
    fn execute(&self, params: &cmd::CmdParams) -> anyhow::Result<()> {
        if params.args.len() < 2 {
            anyhow::bail!("Missing args: <target-id> <backup-id>");
        }

        let target_id = params.args[0].to_string();
        let backup_id = params.args[1].to_string();

        let backup_id = backup_id
            .parse::<u32>()
            .context(format!("Invalid backup ID ('{backup_id}')."))?;

        let mut repo = FileStorageTargetRepository::new(&params.basedir);
        let target = ShowTargetAdapter::new(&repo)
            .execute(&target_id)
            .context(format!("Target not found ('{target_id}')"))?;

        let entry = target
            .backups
            .iter()
            .find(|be| be.id == backup_id)
            .context(format!("Backup not found ('{backup_id}')."))?;

        println!("* Target: {}", target.name);
        println!("ID    : {}", target.id);
        println!("Path  : {}", target.path.to_string_lossy());
        println!();
        println!("* Backup: {:0>3}", entry.id);
        println!("Backup date: {}", entry.timestamp.to_rfc3339());
        println!("Backup file: {}", entry.path.to_string_lossy());
        println!("Note       : {}", entry.note);
        println!();
        println!("##### Delete confirmation #####");
        println!("Do you want to delete the Backup {:0>3}?", entry.id);
        println!("This action cannnot be undone.");
        print!("[yes/No] > ");

        std::io::stdout().flush().unwrap();
        let mut yesno = String::new();
        let reader = (self.create_reader)();
        let mut buf_reader = std::io::BufReader::new(reader);
        buf_reader
            .read_line(&mut yesno)
            .expect("Error: Failed to read line.");
        let yesno = yesno.trim().to_lowercase();
        if yesno != "yes" {
            println!("Cancelled.");
            return Ok(());
        }

        // Delete the backup entry.
        let mut usecase = DeleteBackupUsecase::new(&mut repo);
        let de = usecase.execute(&target.id, entry.id)?;

        println!("The backup[{:0>3}] has been deleted.", de.id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cmd::*;
    use dirback::infra::repository::file_storage::FileStorageTargetRepository;
    use dirback::internal::TargetRepository;

    fn prepare_test_data(temp: &mktemp::TempDir) -> String {
        let mut repo = FileStorageTargetRepository::new(&temp.path());
        let target_path = std::fs::canonicalize(".").unwrap();
        let mut target = repo.add("TestTarget", &target_path).unwrap();

        // Create backups
        let bkdir = repo.make_backup_dir_path(&target);
        let mut bkpaths = Vec::new();
        for _ in 1..=3 {
            let entry = target.new_backup_entry(&bkdir, "tar.gz");
            let _ = std::fs::File::create(&entry.path); // make dummy backup.
            bkpaths.push(entry.path.clone());
            let _ = target.register_backup_entry(entry);
        }
        let target = repo.update(&target).unwrap();

        target.id
    }

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Test target
        let repo = FileStorageTargetRepository::new(&basedir);
        let target_id = prepare_test_data(&temp);
        let target = repo.load(&target_id).unwrap();

        let del_target_bk_path = target.backups[1].path.clone();
        let before_backup_count = target.backups.len();

        // CmdParams
        let args: Vec<String> = ["test", "delete", &target.id, "2"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &basedir).unwrap();

        // Execute
        let cmd = DeleteBackup {
            create_reader: || {
                let data = "yes";
                Box::new(std::io::Cursor::new(data))
            },
        };

        let result = cmd.execute(&params);
        if let Err(ref e) = result {
            println!("{:?}", e);
        }
        assert!(result.is_ok());
        assert!(
            !del_target_bk_path.exists(),
            "The backup file should be deleted."
        );

        let target = repo.load(&target.id).unwrap();
        assert_eq!(target.backups.len(), before_backup_count - 1);
    }

    #[test]
    fn it_cancellable() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Test target
        let repo = FileStorageTargetRepository::new(&basedir);
        let target_id = prepare_test_data(&temp);
        let target = repo.load(&target_id).unwrap();

        let del_target_bk_path = target.backups[1].path.clone();
        let before_backup_count = target.backups.len();

        // CmdParams
        let args: Vec<String> = ["test", "delete", &target.id, "2"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &basedir).unwrap();

        // Execute
        let cmd = DeleteBackup {
            create_reader: || {
                let data = "no";
                Box::new(std::io::Cursor::new(data))
            },
        };

        let result = cmd.execute(&params);
        if let Err(ref e) = result {
            println!("{:?}", e);
        }
        assert!(result.is_ok());
        assert!(
            del_target_bk_path.exists(),
            "The backup file should not be deleted."
        );

        let target = repo.load(&target.id).unwrap();
        assert_eq!(target.backups.len(), before_backup_count);
    }

    #[test]
    fn it_returns_err_when_non_existent_target_id() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // CmdParams
        let args: Vec<String> = ["test", "delete", "xxxx-xxxxx-xxx", "1"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &basedir).unwrap();

        // Execute
        let cmd = DeleteBackup {
            create_reader: || {
                let data = "yes";
                Box::new(std::io::Cursor::new(data))
            },
        };

        let result = cmd.execute(&params);
        assert!(result.is_err());
    }

    #[test]
    fn it_returns_err_when_non_existent_backup_id() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Test target
        let repo = FileStorageTargetRepository::new(&basedir);
        let target_id = prepare_test_data(&temp);
        let target = repo.load(&target_id).unwrap();

        // CmdParams
        let args: Vec<String> = ["test", "delete", &target.id, "10"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &basedir).unwrap();

        // Execute
        let cmd = DeleteBackup {
            create_reader: || {
                let data = "yes";
                Box::new(std::io::Cursor::new(data))
            },
        };

        let result = cmd.execute(&params);
        assert!(result.is_err());
    }
}

//!
//! # DeleteTarget command
//!

use anyhow::Context;
use dirback::adapter::GetTargetAdapter;
use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::usecase::delete_target::DeleteTargetUsecase;

use std::io::BufRead;
use std::io::Write;

pub struct DeleteTarget {
    create_reader: fn() -> Box<dyn std::io::Read>,
}

impl DeleteTarget {
    pub fn new() -> Self {
        Self {
            create_reader: || Box::new(std::io::stdin()),
        }
    }
}

impl dirback_cmd::Command for DeleteTarget {
    fn execute(&self, params: &dirback_cmd::CmdParams) -> anyhow::Result<()> {
        if params.args.is_empty() {
            anyhow::bail!("Missing args: <target-id>");
        }

        let target_id = params.args[0].to_string();

        let mut repo = FileStorageTargetRepository::new(&params.basedir);
        let target = GetTargetAdapter::new(&repo)
            .execute(&target_id)
            .context(format!("Target not found ('{target_id}')"))?;

        println!("* Target: {}", target.name);
        println!("ID    : {}", target.id);
        println!("Path  : {}", target.path.to_string_lossy());
        println!();

        if target.backups.is_empty() {
            println!("No backups yet.");
        } else {
            println!("* Backups ({})", target.backups.len());
            for entry in target.backups.iter() {
                print!("- {}: {}", entry.id, entry.timestamp.to_rfc3339());
                if !entry.note.is_empty() {
                    print!(" - {}", entry.note);
                }
                println!();
            }
            println!();
        }

        println!("##### Delete confirmation #####");
        println!("Do you want to delete the Target '{}'?", target.name);
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

        // Delete the target.
        let mut usecase = DeleteTargetUsecase::new(&mut repo);
        let dt = usecase.execute(&target.id)?;

        println!("The target '{}' has been deleted.", dt.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirback::infra::repository::file_storage::FileStorageTargetRepository;
    use dirback::internal::TargetRepository;
    use dirback_cmd::*;

    fn prepare_test_data(temp: &mktemp::TempDir) -> Vec<String> {
        let mut repo = FileStorageTargetRepository::new(&temp.path());
        let mut ids = Vec::new();

        for i in 1..=3 {
            let target_name = format!("TestTarget{0:>2}", i).to_string();
            let target_path = std::fs::canonicalize(".").unwrap();
            let target = repo.add(&target_name, &target_path).unwrap();
            ids.push(target.id.clone());
        }

        ids
    }

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Test targets
        let repo = FileStorageTargetRepository::new(&basedir);
        let ids = prepare_test_data(&temp);

        // Delete target
        let del_target = repo.load(&ids[1]).unwrap();
        let del_target_dir = basedir.join("targets").join(&del_target.id);

        // CmdParams
        let args: Vec<String> = ["test", "delete-target", &del_target.id]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &basedir).unwrap();

        // Execute
        let cmd = DeleteTarget {
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
            !del_target_dir.exists(),
            "The target info directory should be deleted."
        );

        let result = repo.load(&del_target.id);
        assert!(result.is_none());
    }

    #[test]
    fn it_cancellable() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Test targets
        let repo = FileStorageTargetRepository::new(&basedir);
        let ids = prepare_test_data(&temp);

        // Delete target
        let del_target = repo.load(&ids[1]).unwrap();
        let del_target_dir = basedir.join("targets").join(&del_target.id);

        // CmdParams
        let args: Vec<String> = ["test", "delete-target", &del_target.id]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &basedir).unwrap();

        // Execute
        let cmd = DeleteTarget {
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
            del_target_dir.exists(),
            "The target info directory should not be deleted."
        );

        let result = repo.load(&del_target.id);
        assert!(result.is_some());
    }

    #[test]
    fn it_returns_err_when_non_existent_target_id() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // CmdParams
        let args: Vec<String> = ["test", "delete-target", "xxxx-xxxx-xxxx"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let params = CmdParams::build(&args, &basedir).unwrap();

        // Execute
        let cmd = DeleteTarget {
            create_reader: || {
                let data = "yes";
                Box::new(std::io::Cursor::new(data))
            },
        };

        let result = cmd.execute(&params);
        assert!(result.is_err());
    }
}

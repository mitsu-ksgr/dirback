//!
//! # ShowTarget command
//!

use dirback::adapter::GetTargetAdapter;
use dirback::infra::repository::file_storage::FileStorageTargetRepository;

pub struct ShowTarget;

impl cmd::Command for ShowTarget {
    fn execute(&self, params: &cmd::CmdParams) -> anyhow::Result<()> {
        if params.args.is_empty() {
            anyhow::bail!("Missing args: <target_id>");
        }

        let target_id = params.args[0].to_string();

        let repo = FileStorageTargetRepository::new(&params.basedir);
        let adapter = GetTargetAdapter::new(&repo);

        if let Some(target) = adapter.execute(&target_id) {
            println!("* Target Information");
            println!("ID            : {}", target.id);
            println!("Name          : {}", target.name);
            println!("Path          : {}", target.path.to_string_lossy());
            println!("Backup count  : {}", target.backups.len());

            if !target.backups.is_empty() {
                println!("\n* Backups");
                for entry in target.backups {
                    print!("{:0>3}: {}", entry.id, entry.timestamp.to_rfc3339());

                    if !entry.note.is_empty() {
                        println!(" # {}", entry.note);
                    } else {
                        println!();
                    }
                }
            }
        } else {
            println!("Target is none (id={target_id}).");
        }

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
        let _ = repo.add("TestTarget", std::path::Path::new(".")).unwrap();

        // CmdParams
        let args: Vec<String> = ["test", "show", ""].iter().map(|s| s.to_string()).collect();

        let params = CmdParams::build(&args, &basedir).unwrap();

        let cmd = ShowTarget {};
        let result = cmd.execute(&params);
        assert!(result.is_ok());
    }
}

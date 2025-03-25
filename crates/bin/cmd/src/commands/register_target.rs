//!
//! # RegisterTarget command
//!

use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::usecase::register_target::RegisterTargetUsecase;

pub struct RegisterTarget;

impl cmd::Command for RegisterTarget {
    fn execute(&self, params: &cmd::CmdParams) -> anyhow::Result<()> {
        if params.args.len() < 2 {
            anyhow::bail!("Missing args: <name> <path>");
        }

        let name = params.args[0].to_string();
        let path = std::path::PathBuf::from(params.args[1].to_string());

        if !path.exists() {
            anyhow::bail!("Target path is invalid: '{}'", path.to_string_lossy());
        }

        let path = std::fs::canonicalize(&path)?;

        let mut repo = FileStorageTargetRepository::new(&params.basedir);
        let mut usecase = RegisterTargetUsecase::new(&mut repo);

        let target = usecase.execute(&name, &path)?;

        println!("A new target has been registered!");
        println!("ID  : {}", target.id);
        println!("Name: {}", target.name);
        println!("Path: {}", target.path.to_string_lossy());

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
        let args: Vec<String> = ["test", "register", "test-target", "."]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let params = CmdParams::build(&args, &basedir).unwrap();

        let cmd = RegisterTarget {};
        let result = cmd.execute(&params);
        assert!(result.is_ok());

        let repo = FileStorageTargetRepository::new(&basedir);
        let targets = repo.load_all().unwrap();
        assert_eq!(targets.len(), 1);
    }
}

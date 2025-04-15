//!
//! # DeleteTarget command
//!

use crate::commands::Command;

use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::usecase::delete_target::DeleteTargetUsecase;
use dirback::usecase::dto::Target;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteTargetPayload {
    pub target_id: String,
}

pub struct DeleteTarget;

impl Command for DeleteTarget {
    type Payload = DeleteTargetPayload;
    type Output = Target;

    fn execute(
        &self,
        datadir: &std::path::Path,
        payload: Self::Payload,
    ) -> anyhow::Result<Self::Output> {
        if payload.target_id.is_empty() {
            anyhow::bail!("Target not found");
        }

        let mut repo = FileStorageTargetRepository::new(datadir);
        let mut usecase = DeleteTargetUsecase::new(&mut repo);
        let target = usecase.execute(&payload.target_id)?;

        Ok(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirback::internal::TargetRepository;

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Test target
        let mut repo = FileStorageTargetRepository::new(&basedir);
        let target = repo.add("TestTarget", std::path::Path::new(".")).unwrap();

        // Command
        let cmd = DeleteTarget;
        let payload = DeleteTargetPayload {
            target_id: target.id.clone(),
        };

        assert_eq!(repo.load_all().unwrap().len(), 1);

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_ok());

        let got = result.unwrap();
        assert_eq!(got.id, target.id);
        assert_eq!(got.name, target.name);
        assert_eq!(repo.load_all().unwrap().len(), 0);
    }

    #[test]
    fn it_returns_err_when_target_not_found() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Command
        let cmd = DeleteTarget;
        let payload = DeleteTargetPayload {
            target_id: String::from("xxxxx-xxxxx-xxxxx"),
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_err());
    }
}

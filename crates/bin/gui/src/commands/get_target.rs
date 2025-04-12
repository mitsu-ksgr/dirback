//!
//! # GetTarget command
//!

use crate::commands::Command;

use dirback::adapter::GetTargetAdapter;
use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::usecase::dto::Target;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetTargetPayload {
    pub target_id: String,
}

pub struct GetTarget;

impl Command for GetTarget {
    type Payload = GetTargetPayload;
    type Output = Option<Target>;

    fn execute(
        &self,
        datadir: &std::path::Path,
        payload: Self::Payload,
    ) -> anyhow::Result<Self::Output> {
        if payload.target_id.is_empty() {
            return Ok(None);
        }

        let repo = FileStorageTargetRepository::new(datadir);
        let adapter = GetTargetAdapter::new(&repo);
        Ok(adapter.execute(&payload.target_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirback::infra::repository::file_storage::FileStorageTargetRepository;
    use dirback::internal::TargetRepository;

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Test target
        let mut repo = FileStorageTargetRepository::new(&basedir);
        let target = repo.add("TestTarget", std::path::Path::new(".")).unwrap();

        // Command
        let cmd = GetTarget;
        let payload = GetTargetPayload {
            target_id: target.id.clone(),
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_some());

        let got = result.unwrap();
        assert_eq!(got.id, target.id);
        assert_eq!(got.name, target.name);
    }

    #[test]
    fn it_returns_none_when_target_is_none() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Command
        let cmd = GetTarget;
        let payload = GetTargetPayload {
            target_id: String::from("xxxxx-xxxxx-xxxxx"),
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_none());
    }
}

//!
//! # RegisterTarget command
//!

use crate::commands::Command;

use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::usecase::dto::Target;
use dirback::usecase::register_target::RegisterTargetUsecase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterTargetPayload {
    pub name: String,
    pub path: std::path::PathBuf,
}

pub struct RegisterTarget;

impl Command for RegisterTarget {
    type Payload = RegisterTargetPayload;
    type Output = Target;

    fn execute(
        &self,
        datadir: &std::path::Path,
        payload: Self::Payload,
    ) -> anyhow::Result<Self::Output> {
        if payload.name.is_empty() {
            anyhow::bail!("Error: name is empty.");
        }

        if !payload.path.exists() {
            anyhow::bail!(
                "Error: target path is invalid: '{}'",
                payload.path.display()
            );
        }

        let mut repo = FileStorageTargetRepository::new(datadir);
        let mut usecase = RegisterTargetUsecase::new(&mut repo);

        let target = usecase.execute(&payload.name, &payload.path)?;
        Ok(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Command
        let cmd = RegisterTarget;
        let payload = RegisterTargetPayload {
            name: String::from("Test Target"),
            path: std::path::PathBuf::from("."),
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_ok());

        let got = result.unwrap();
        assert_eq!(got.name, "Test Target");
        assert_eq!(got.path, std::path::PathBuf::from("."));
    }

    #[test]
    fn it_returns_err_when_name_is_missing() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Command
        let cmd = RegisterTarget;
        let payload = RegisterTargetPayload {
            name: String::from(""),
            path: std::path::PathBuf::from("."),
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_err());
    }

    #[test]
    fn it_returns_err_when_invalid_path() {
        let temp = mktemp::TempDir::new().unwrap();
        let basedir = temp.path();

        // Command
        let cmd = RegisterTarget;
        let payload = RegisterTargetPayload {
            name: String::from("Test Target"),
            path: temp.path().join("inavlid-path"),
        };

        let result = cmd.execute(&basedir, payload);
        assert!(result.is_err());
    }
}

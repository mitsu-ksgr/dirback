//!
//! # ListTargets command
//!

use crate::commands::{Command, NoPayload};

use dirback::adapter::ListTargetsAdapter;
use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::usecase::dto::Target;

pub struct ListTargets;

impl Command for ListTargets {
    type Payload = NoPayload;
    type Output = Vec<Target>;

    fn execute(
        &self,
        datadir: &std::path::Path,
        _payload: Self::Payload,
    ) -> anyhow::Result<Self::Output> {
        let repo = FileStorageTargetRepository::new(datadir);
        let adapter = ListTargetsAdapter::new(&repo);
        let targets = adapter.execute()?;
        Ok(targets)
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
        let _ = repo.add("TestTarget1", std::path::Path::new(".")).unwrap();
        let _ = repo.add("TestTarget2", std::path::Path::new(".")).unwrap();

        // Command
        let cmd = ListTargets;

        let result = cmd.execute(&basedir, NoPayload);
        assert!(result.is_ok());

        let got = result.unwrap();
        assert_eq!(got.len(), 2);
    }
}

//!
//! # Register target usecase
//!
//! TOOD: Probably not necessary.
//!

use crate::domain::repository::targets::TargetRepository;
use std::path::Path;

pub struct RegisterTargetUsecase<R: TargetRepository> {
    repo: R,
}

impl<R: TargetRepository> RegisterTargetUsecase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn execute(&mut self, target_name: &str, target_path: &Path) -> anyhow::Result<()> {
        self.repo.add(target_name, &target_path)?;
        Ok(())
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::repository::in_memory::InMemoryTargetRepository;

    #[test]
    fn it_works() {
        let repo = InMemoryTargetRepository::new();
        let mut usecase = RegisterTargetUsecase::new(repo);

        let target_name = "Test Target";
        let target_path = Path::new("/tmp/path/to/tagret");

        let targets = usecase.repo.load_all().unwrap();
        assert_eq!(targets.len(), 0);

        let result = usecase.execute(target_name, &target_path);
        assert!(result.is_ok());

        let targets = usecase.repo.load_all().unwrap();
        assert_eq!(targets.len(), 1);
    }
}

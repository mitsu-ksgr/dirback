//!
//! # Register target usecase
//!

use crate::domain::repository::targets::TargetRepository;
use crate::usecase::dto::Target;
use std::path::Path;

pub struct RegisterTargetUsecase<'a, R: TargetRepository> {
    repo: &'a mut R,
}

impl<'a, R: TargetRepository> RegisterTargetUsecase<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn execute(&mut self, target_name: &str, target_path: &Path) -> anyhow::Result<Target> {
        let target = self.repo.add(target_name, target_path)?;
        Ok(target.into())
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
        let mut repo = InMemoryTargetRepository::new();
        let mut usecase = RegisterTargetUsecase::new(&mut repo);

        let target_name = "Test Target";
        let target_path = Path::new("/tmp/path/to/target");

        let targets = usecase.repo.load_all().unwrap();
        assert_eq!(targets.len(), 0);

        let result = usecase.execute(target_name, &target_path);
        assert!(result.is_ok());

        let target = result.unwrap();
        assert_eq!(target.name, "Test Target");
        assert_eq!(target.path, Path::new("/tmp/path/to/target"));

        let targets = usecase.repo.load_all().unwrap();
        assert_eq!(targets.len(), 1);
    }
}

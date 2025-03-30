//!
//! # Delete target usecase
//!

use crate::domain::repository::targets::TargetRepository;
use crate::usecase::dto::Target;

pub struct DeleteTargetUsecase<'a, R: TargetRepository> {
    repo: &'a mut R,
}

impl<'a, R: TargetRepository> DeleteTargetUsecase<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn execute(&mut self, target_id: &str) -> anyhow::Result<Target> {
        let target = self.repo.delete_target(target_id)?;
        Ok(target.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::repository::in_memory::InMemoryTargetRepository;

    #[test]
    fn it_works() {
        let mut repo = InMemoryTargetRepository::new();

        let target = repo.add("TestTarget", std::path::Path::new(".")).unwrap();
        let targets = repo.load_all().unwrap();
        assert_eq!(targets.len(), 1);

        let mut usecase = DeleteTargetUsecase::new(&mut repo);
        let result = usecase.execute(&target.id);
        assert!(result.is_ok());

        let targets = repo.load_all().unwrap();
        assert_eq!(targets.len(), 0);
    }
}

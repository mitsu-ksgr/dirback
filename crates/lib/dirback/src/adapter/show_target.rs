//!
//! # Adapter: ShowTarget
//!

use crate::domain::repository::targets::TargetRepository;
use crate::usecase::dto::Target;

pub struct ShowTargetAdapter<'a, R: TargetRepository> {
    repo: &'a R,
}

impl<'a, R: TargetRepository> ShowTargetAdapter<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    pub fn execute(&self, target_id: &str) -> Option<Target> {
        self.repo.load(target_id).map(|target| target.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::repository::in_memory::InMemoryTargetRepository;

    #[test]
    fn it_works() {
        let mut repo = InMemoryTargetRepository::new();
        let mut ids = Vec::new();
        for i in 1..=3 {
            let name = format!("Test Target {i}");
            let path = std::path::PathBuf::from(format!("target{i}"));
            let target = repo.add(&name, &path).unwrap();
            ids.push(target.id.clone());
        }

        let adapter = ShowTargetAdapter::new(&repo);
        let target = adapter.execute(&ids[0]);
        assert!(target.is_some());

        let target = target.unwrap();
        assert_eq!(target.id, ids[0]);
        assert_eq!(target.name, "Test Target 1");
        assert_eq!(target.path, std::path::PathBuf::from("target1"));
        assert_eq!(target.backups.len(), 0);
    }

    #[test]
    fn it_return_none_if_non_existing_target_id() {
        let mut repo = InMemoryTargetRepository::new();
        let adapter = ShowTargetAdapter::new(&repo);
        let target = adapter.execute("non-existing-target-id");
        assert!(target.is_none());
    }
}

//!
//! # Adapter: ListTarget
//!

use crate::domain::repository::targets::TargetRepository;
use crate::usecase::dto::Target;

pub struct ListTargetsAdapter<'a, R: TargetRepository> {
    repo: &'a R,
}

impl<'a, R: TargetRepository> ListTargetsAdapter<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    pub fn execute(&self) -> anyhow::Result<Vec<Target>> {
        let targets = self.repo.load_all()?;

        Ok(targets.into_iter().map(Target::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::repository::in_memory::InMemoryTargetRepository;

    #[test]
    fn it_works() {
        let mut repo = InMemoryTargetRepository::new();
        for i in 1..=3 {
            let name = format!("Test Target {i}");
            let path = std::path::PathBuf::from(format!("target{i}"));
            let _ = repo.add(&name, &path);
        }

        let adapter = ListTargetsAdapter::new(&repo);
        let targets = adapter.execute();
        assert!(targets.is_ok());

        let targets = targets.unwrap();
        assert_eq!(targets.len(), 3);
    }
}

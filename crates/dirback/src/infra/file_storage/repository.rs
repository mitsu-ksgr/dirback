//!
//! # FileStorage Repository
//!
//! Store target informations to file storage.
//!

use crate::domain::model::target::Target;
use crate::domain::repository::targets::TargetRepository;
use std::path::Path;

pub struct FileStorageTargetRepository {}

impl FileStorageTargetRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl TargetRepository for FileStorageTargetRepository {
    fn load_all(&self) -> anyhow::Result<Vec<Target>> {
        anyhow::bail!("Not implemented yet");
    }

    fn load(&self, target_id: &str) -> Option<Target> {
        None
    }

    fn update(&mut self, target: &Target) -> Option<Target> {
        None
    }

    fn add(&mut self, name: &str, target_path: &Path) -> anyhow::Result<Target> {
        anyhow::bail!("Not implemented yet");
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_new() {
        let repo = FileStorageTargetRepository::new();
    }
}

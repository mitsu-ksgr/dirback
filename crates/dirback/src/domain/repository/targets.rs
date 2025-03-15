//!
//! # TargetRepository
//!

use crate::domain::model::target::Target;
use std::path::{Path, PathBuf};

pub trait TargetRepository {
    /// Load all target informations.
    fn load_all(&self) -> anyhow::Result<Vec<Target>>;

    /// Load a target information.
    fn load(&self, target_id: &str) -> Option<Target>;

    /// Update a target information.
    fn update(&mut self, target: &Target) -> anyhow::Result<Target>;

    /// Add a new target information.
    fn add(&mut self, name: &str, target_path: &Path) -> anyhow::Result<Target>;

    /// Make a backup path of the target.
    fn make_backup_dir_path(&self, target: &Target) -> PathBuf;
}

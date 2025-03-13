//!
//! # TargetRepository
//!

use crate::domain::model::target::Target;
use std::path::Path;

pub trait TargetRepository {
    /// Load all target informations.
    fn load_all(&self) -> anyhow::Result<Vec<Target>>;

    /// Load a target information.
    fn load(&self, target_id: &str) -> Option<Target>;

    /// Update a target information.
    fn update(&mut self, target: &Target) -> Option<Target>;

    /// Add a new target information.
    fn add(&mut self, name: &str, target_path: &Path) -> anyhow::Result<Target>;
}

//!
//! # Backup Service
//!

use std::path::Path;

pub trait BackupService {
    /// Backup directory.
    fn backup(&self, src: &Path, dest: &Path) -> anyhow::Result<()>;

    /// Restore directory.
    fn restore(&self, src: &Path, dest: &Path) -> anyhow::Result<()>;
}

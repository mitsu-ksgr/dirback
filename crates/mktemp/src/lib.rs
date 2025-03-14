//!
//! # mktemp
//!
//! Create a temporary directory.
//!

use std::path::PathBuf;

/// # Temporary directory.
///
/// Create a new temporary directory.
///
/// the temporary directory will be removed when the instance of TempDir is dropped.
#[derive(Debug)]
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new() -> std::io::Result<Self> {
        let id = uuid::Uuid::new_v4();
        let path = std::env::temp_dir().join(id.to_string());

        std::fs::create_dir_all(&path)?;

        Ok(TempDir { path })
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if !self.path.exists() {
            return;
        }

        let _ = std::fs::remove_dir_all(&self.path);
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tempdir_name_is_uuidv4() {
        let temp = TempDir::new().unwrap();
        let id = temp.path.file_name().unwrap();
        assert_eq!(id.len(), 36, "tempdir name is not uuid v4 ({:?})", id);
    }

    #[test]
    fn temporary_direcotry_have_same_lifetime_as_tempdir_instance() {
        let path;

        {
            let temp = TempDir::new().unwrap();
            path = temp.path();
            assert!(path.exists(), "Temporary directory should be created.");
        }

        assert!(!path.exists(), "Temporary directory should be removed.");
    }
}

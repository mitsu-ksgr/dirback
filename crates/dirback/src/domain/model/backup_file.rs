//!
//! # Backup file
//!
//! Backup file information.
//!

use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

//-----------------------------------------------------------------------------
// Errors
//-----------------------------------------------------------------------------
#[derive(Debug)]
enum BackupFileError {
    MissingFilename(String),
    InvalidTimestamp(String),
}

impl std::fmt::Display for BackupFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BackupFileError::MissingFilename(filepath) => {
                write!(f, "Missing filename: `{}`", filepath)
            },
            BackupFileError::InvalidTimestamp(filepath) => {
                write!(f, "Invalid timestamp in filename: `{}`", filepath)
            },
        }
    }
}

impl std::error::Error for BackupFileError {}


//-----------------------------------------------------------------------------
//  BackupFile
//-----------------------------------------------------------------------------
#[derive(Debug)]
pub struct BackupFile {
    pub path: PathBuf,
    pub timestamp: DateTime<Utc>,
}

impl BackupFile {
    /// Create a new BackupFile
    pub fn new(backup_filepath: &Path, timestamp: DateTime<Utc>) -> Self {
        BackupFile {
            path: backup_filepath.to_path_buf(),
            timestamp
        }
    }

    pub fn build(backup_filepath: &Path) -> Result<Self, BackupFileError> {
        let filestem = backup_filepath
            .file_stem()
            .ok_or_else(|| BackupFileError::MissingFilename(
                backup_filepath.to_string_lossy().to_string()
            ))?;
        let filestem = filestem.to_string_lossy().to_string();

        let timestamp = DateTime::parse_from_str(&filestem, "%Y%m%dT%H%M%S%z")
            .map_err(|_| BackupFileError::InvalidTimestamp(
                    backup_filepath.to_string_lossy().to_string()
            ))?
            .with_timezone(&Utc);

        Ok(BackupFile {
            path: backup_filepath.to_path_buf(),
            timestamp,
        })
    }

    /// Make a backup file path.
    pub fn make_backup_filepath(
        dirpath: &Path,
        backup_date: &DateTime<Utc>,
    ) -> PathBuf {
        let mut path = dirpath.to_path_buf();
        path.push(backup_date.format("%Y%m%dT%H%M%SZ.tar.gz").to_string());
        path
    }

    // TODO
}


//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_backup_filepath() {
        //> 2025-01-23 03:24:56.789 UTC
        let backup_date = DateTime::parse_from_str(
            "2025-01-23 12:24:56.789 +09:00",
            "%Y-%m-%d %H:%M:%S%.3f %z"
        ).unwrap().to_utc();
        let dirpath = Path::new("/tmp/dirback");

        let bkpath = BackupFile::make_backup_filepath(
            dirpath,
            &backup_date
        );

        assert_eq!(
            bkpath.to_str(),
            Some("/tmp/dirback/20250123T032456Z.tar.gz")
        );
    }

    #[test]
    fn build_missing_filename() {
        let bkpath = Path::new("/");
        let result = BackupFile::build(bkpath);

        assert!(result.is_err());
        assert!(matches!(
                result.unwrap_err(),
                BackupFileError::MissingFilename(_)));
    }

    #[test]
    fn build_invalid_timestamp() {
        let bkpath = Path::new("/tmp/dirback/bhge.tar.gz");
        let result = BackupFile::build(bkpath);

        assert!(result.is_err());
        assert!(matches!(
                result.unwrap_err(),
                BackupFileError::InvalidTimestamp(_)));
    }
}


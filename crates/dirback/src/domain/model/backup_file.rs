//! # Backup file
//!
//! Backup file information.
//!

use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};

//-----------------------------------------------------------------------------
// Errors
//-----------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub enum BackupFileError {
    MissingFilename(String),
    InvalidFiletype(String),
    InvalidTimestamp(String),
}

impl std::fmt::Display for BackupFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BackupFileError::MissingFilename(filepath) => {
                write!(f, "Missing filename: `{}`", filepath)
            }
            BackupFileError::InvalidFiletype(filepath) => {
                write!(f, "Invalid filetype(tar.gz only): `{}`", filepath)
            }
            BackupFileError::InvalidTimestamp(filepath) => {
                write!(f, "Invalid timestamp in filename: `{}`", filepath)
            }
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
            timestamp,
        }
    }

    pub fn build(backup_filepath: &Path) -> Result<Self, BackupFileError> {
        let filename = backup_filepath
            .file_name()
            .and_then(|s| s.to_str())
            .map(String::from)
            .ok_or_else(|| {
                BackupFileError::MissingFilename(backup_filepath.to_string_lossy().to_string())
            })?;

        if !filename.ends_with(".tar.gz") {
            return Err(BackupFileError::InvalidFiletype(
                backup_filepath.to_string_lossy().to_string(),
            ));
        }

        let filestem = filename.trim_end_matches(".tar.gz").to_string();
        let timestamp = &filestem.replace("Z", "+00:00");
        let timestamp = DateTime::parse_from_str(timestamp, "%Y%m%dT%H%M%S%z")
            .map_err(|_| {
                BackupFileError::InvalidTimestamp(backup_filepath.to_string_lossy().to_string())
            })?
            .with_timezone(&Utc);

        Ok(BackupFile {
            path: backup_filepath.to_path_buf(),
            timestamp,
        })
    }

    /// Make a backup file path.
    pub fn make_backup_filepath(dirpath: &Path, backup_date: &DateTime<Utc>) -> PathBuf {
        let mut path = dirpath.to_path_buf();
        path.push(backup_date.format("%Y%m%dT%H%M%SZ.tar.gz").to_string());
        path
    }

    /// Returns a path as String.
    pub fn path(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
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
        let backup_date =
            DateTime::parse_from_str("2025-01-23 12:24:56.789 +09:00", "%Y-%m-%d %H:%M:%S%.3f %z")
                .unwrap()
                .to_utc();
        let dirpath = Path::new("/tmp/dirback");

        let bkpath = BackupFile::make_backup_filepath(dirpath, &backup_date);

        assert_eq!(
            bkpath.to_str(),
            Some("/tmp/dirback/20250123T032456Z.tar.gz")
        );
    }

    #[test]
    fn build() {
        let filepath = String::from("/tmp/dirback/20250123T032456Z.tar.gz");
        let bkpath = Path::new(&filepath);

        let result = BackupFile::build(bkpath);
        if let Err(e) = result {
            panic!("Expected Ok, but got Err: {:?}", e);
        }

        let bkfile = result.unwrap();
        assert_eq!(bkfile.path.to_string_lossy().to_string(), filepath.clone());
        assert_eq!(
            bkfile.timestamp,
            DateTime::parse_from_rfc3339("2025-01-23T03:24:56Z").expect("")
        );
    }

    #[test]
    fn path_method() {
        let filepath = String::from("/tmp/dirback/20250123T032456Z.tar.gz");
        let bkpath = Path::new(&filepath);
        let result = BackupFile::build(bkpath).unwrap();

        assert_eq!(result.path(), filepath.clone());
    }

    #[test]
    fn build_missing_filename() {
        let filepath = String::from("/");
        let bkpath = Path::new(&filepath);
        let result = BackupFile::build(bkpath);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            BackupFileError::MissingFilename(filepath.clone())
        );
    }

    #[test]
    fn build_invalid_filetype() {
        let filepath = String::from("/tmp/dirback/20250123T032456Z.zip");
        let bkpath = Path::new(&filepath);
        let result = BackupFile::build(bkpath);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            BackupFileError::InvalidFiletype(filepath.clone())
        );
    }

    #[test]
    fn build_invalid_timestamp() {
        let filepath = String::from("/tmp/dirback/bhge.tar.gz");
        let bkpath = Path::new(&filepath);
        let result = BackupFile::build(bkpath);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            BackupFileError::InvalidTimestamp(filepath.clone())
        );
    }
}

//!
//! # tar.gz backup service
//!

use crate::domain::service::backup_service::BackupService;
use std::path::Path;

pub struct TargzBackupService {}

impl TargzBackupService {
    pub fn new() -> Self {
        Self {}
    }
}

impl BackupService for TargzBackupService {
    fn backup(&self, src: &Path, dest: &Path) -> anyhow::Result<()> {
        targz::archive(src, dest)
    }

    fn restore(&self, src: &Path, dest: &Path) -> anyhow::Result<()> {
        targz::extract(src, dest)
    }
}

//-----------------------------------------------------------------------------
// Tests
//-----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let service = TargzBackupService::new();
        let ts = chrono::Utc::now().to_rfc3339();
        let ts_filename = "timestamp.txt";

        // Make test file (test/timestamp.txt).
        let temp = mktemp::TempDir::new().unwrap();
        let test_dir = temp.path().join("origin");
        let _ = std::fs::create_dir_all(&test_dir);
        let _ = std::fs::write(&test_dir.join(ts_filename), &ts);

        // Archive test.
        let targz = temp.path().join("test.tar.gz");
        let service = TargzBackupService::new();
        let result = service.backup(&test_dir, &targz);
        assert!(result.is_ok());
        assert!(targz.exists(), "test.tar.gz should be created.");

        // Extract test.
        let extr_dir = temp.path().join("extract");
        let result = service.restore(&targz, &extr_dir);
        assert!(result.is_ok());
        assert!(
            extr_dir.join(ts_filename).exists(),
            "{} file should be restored.",
            ts_filename
        );

        let src_content = std::fs::read_to_string(&test_dir.join(ts_filename)).unwrap();
        let ext_content = std::fs::read_to_string(&extr_dir.join(ts_filename)).unwrap();
        assert_eq!(src_content, ext_content);

        /*---------- test -----------
        let output = std::process::Command::new("/bin/tree")
            .arg(&temp.path())
            .arg("-a")
            .output()
            .unwrap();
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            println!("* Tree");
            println!("{s}");
        }
        //---------- test -----------*/
    }
}

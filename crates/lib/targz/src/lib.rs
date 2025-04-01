//!
//! # targz
//!
//! .tar.gz file archiver / extractor.
//!

use std::io::Write;
use std::path::Path; // Required to flush tar data to disk.

/// Archive the specified directory as a tar.gz file.
///
/// - src ... Path to the directory to be archived.
/// - dest ... Output destination of archive file.
pub fn archive(src: &Path, dest: &Path) -> anyhow::Result<()> {
    let temp = mktemp::TempDir::new()?;
    let temp_dest = temp.path().join("temp.tar.gz");

    // Make destination file (tar.gz).
    let targz = std::fs::File::create(&temp_dest)?;
    let enc = flate2::write::GzEncoder::new(targz, flate2::Compression::default());
    let mut ar = tar::Builder::new(enc);

    // Add directory to archive.
    ar.append_dir_all(".", src)?;

    // Flush data to disk.
    let mut enc = ar.into_inner()?;
    enc.flush()?;

    // Copy to dest
    std::fs::copy(&temp_dest, dest)?;
    std::fs::remove_file(&temp_dest)?;

    Ok(())
}

/// Extracts the tar.gz file to the specified path.
///
/// - src ... tar.gz file
/// - dest ... Path of the destination directory.
pub fn extract(src: &Path, dest: &Path) -> anyhow::Result<()> {
    let targz = std::fs::File::open(src)?;
    let dec = flate2::read::GzDecoder::new(std::io::BufReader::new(targz));

    // extract a tar archive
    let mut ar = tar::Archive::new(dec);
    ar.unpack(dest)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::path::{Path, PathBuf};

    fn list_entry(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.push(path.clone());
                files.extend(list_entry(&path)?);
            } else {
                files.push(path);
            }
        }
        files.sort();
        Ok(files)
    }

    // Make test dirs
    //
    // {temp-dir}
    // └── sample
    //     ├── foo
    //     │   ├── bar
    //     │   │   ├── baz
    //     │   │   └── baz.txt
    //     │   ├── bar.txt
    //     │   └── .hiddenfile
    //     └── foo.txt
    fn prepare_test_dir_and_files(temp: &mktemp::TempDir) {
        let dir = temp.path().join("sample");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::File::create(dir.join("foo.txt"));

        let dir = dir.join("foo");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::File::create(dir.join("bar.txt"));
        let _ = std::fs::File::create(dir.join(".hiddenfile"));

        let dir = dir.join("bar");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::File::create(dir.join("baz.txt"));

        let dir = dir.join("baz");
        let _ = std::fs::create_dir_all(&dir);
    }

    #[test]
    fn it_works() {
        let temp = mktemp::TempDir::new().unwrap();

        // Make sample files
        prepare_test_dir_and_files(&temp);
        let sample = temp.path().join("sample");
        let before_dirs = list_entry(&sample).unwrap();

        // Archive test
        let targz = temp.path().join("test.tar.gz");
        let result = archive(&sample, &targz);
        assert!(result.is_ok());
        assert!(targz.exists(), "test.tar.gz should be created.");

        // Extract test
        let _ = std::fs::remove_dir_all(&sample);
        let result = extract(&targz, &sample);
        assert!(result.is_ok(), "Result: {:?}", result);
        assert!(sample.exists(), "sample directory should be created.");

        let _ = std::fs::remove_file(&targz);
        let after_dirs = list_entry(&sample).unwrap();
        assert_eq!(after_dirs, before_dirs);
    }

    mod archive {
        use super::*;

        #[test]
        fn it_returns_error_if_targe_does_not_exists() {
            let temp = mktemp::TempDir::new().unwrap();

            let target = temp.path().join("nonexsistent-target");
            let targz = temp.path().join("test.tar.gz");

            let result = archive(&target, &targz);
            assert!(result.is_err());
            assert!(!targz.exists(), "test.tar.gz should not be created.");
        }
    }

    mod extract {
        use super::*;

        #[test]
        fn it_returns_error_if_targz_file_does_not_exists() {
            let temp = mktemp::TempDir::new().unwrap();

            let targz = temp.path().join("nonexistent.tar.gz");
            let dest = temp.path().join("output");

            let result = extract(&targz, &dest);
            assert!(result.is_err());
            assert!(!dest.exists());
        }
    }
}

//!
//! # Target
//!
//! Backup target.
//! Contains the target directory path and id.
//!

#[derive(Debug)]
pub struct Target {
    pub id: String,
    pub path: String,
    // TODO backupfile list
}

impl Target {
    /// Create a new BackupTarget.
    ///
    /// The id is the identifier of target.
    pub fn new(id: String, path: String) -> Self {
        Target { id, path }
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
        let td = Target::new(String::from("xxx"), String::from("/path/to/target"));

        assert_eq!(td.id, "xxx");
        assert_eq!(td.path, "/path/to/target");
    }
}

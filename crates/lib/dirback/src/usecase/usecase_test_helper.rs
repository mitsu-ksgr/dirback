//!
//! # Usecase Test Helper
//!

use crate::domain::service::backup_service::BackupService;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

#[cfg(test)]
pub struct TestBackupService {
    backup_counter: Rc<RefCell<usize>>,
    restore_counter: Rc<RefCell<usize>>,
}

impl TestBackupService {
    pub fn new() -> (Self, Rc<RefCell<usize>>, Rc<RefCell<usize>>) {
        let backup_counter = Rc::new(RefCell::new(0));
        let restore_counter = Rc::new(RefCell::new(0));

        (
            Self {
                backup_counter: backup_counter.clone(),
                restore_counter: restore_counter.clone(),
            },
            backup_counter,
            restore_counter,
        )
    }
}

impl BackupService for TestBackupService {
    fn backup(&self, _src: &Path, _dest: &Path) -> anyhow::Result<()> {
        *self.backup_counter.borrow_mut() += 1;
        Ok(())
    }

    fn restore(&self, _src: &Path, _dest: &Path) -> anyhow::Result<()> {
        *self.restore_counter.borrow_mut() += 1;
        Ok(())
    }
}

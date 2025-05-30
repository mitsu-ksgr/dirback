//!
//! # commands module
//!

pub mod backup_target;
pub mod command;
pub mod delete_backup;
pub mod delete_target;
pub mod get_target;
pub mod list_targets;
pub mod register_target;
pub mod restore_target;

pub use backup_target::BackupTarget;
pub use command::Command;
pub use command::NoPayload;
pub use delete_backup::DeleteBackup;
pub use delete_target::DeleteTarget;
pub use get_target::GetTarget;
pub use list_targets::ListTargets;
pub use register_target::RegisterTarget;
pub use restore_target::RestoreTarget;

//
// Define command types.
//
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum CommandType {
    GetTarget(get_target::GetTargetPayload),
    ListTargets(NoPayload),
    RegisterTarget(register_target::RegisterTargetPayload),
    DeleteTarget(delete_target::DeleteTargetPayload),
    BackupTarget(backup_target::BackupTargetPayload),
    DeleteBackup(delete_backup::DeleteBackupPayload),
    RestoreTarget(restore_target::RestoreTargetPayload),
}

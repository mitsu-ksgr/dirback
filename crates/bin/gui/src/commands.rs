//!
//! # commands module
//!

pub mod command;
pub mod get_target;
pub mod list_targets;

pub use command::Command;
pub use command::NoPayload;
pub use get_target::GetTarget;
pub use list_targets::ListTargets;

//
// Define command types.
//
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum CommandType {
    GetTarget(get_target::GetTargetPayload),
    ListTargets(NoPayload),
}

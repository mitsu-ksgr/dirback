//!
//! # commands module
//!

pub mod command;
pub mod get_target;
pub mod list_targets;
pub mod register_target;

pub use command::Command;
pub use command::NoPayload;
pub use get_target::GetTarget;
pub use list_targets::ListTargets;
pub use register_target::RegisterTarget;

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
}

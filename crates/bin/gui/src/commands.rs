//!
//! # commands module
//!

pub mod command;
pub mod get_target;

pub use command::Command;
pub use get_target::GetTarget;

//
// Define command types.
//
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum CommandType {
    ListTarget,
    GetTarget(get_target::GetTargetPayload),
}

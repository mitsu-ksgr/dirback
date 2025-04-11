//!
//! # commands module
//!

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Command {
    ListTarget,
    GetTarget { id: String },
}

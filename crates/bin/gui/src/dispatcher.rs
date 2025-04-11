//!
//! # Command dispatcher
//!

use crate::commands::Command;

pub struct Dispatcher {
    pub datadir: std::path::PathBuf,
}

impl Dispatcher {
    pub fn new(datadir: &std::path::Path) -> Self {
        Self {
            datadir: datadir.to_path_buf(),
        }
    }

    pub fn dispatch(&self, cmd: Command) -> Result<serde_json::Value, String> {
        match cmd {
            Command::ListTarget => {
                let result = handle_list_target()?;
                Ok(serde_json::json!({ "message": result }))
            }
            Command::GetTarget { id } => {
                let result = handle_get_target(&id)?;
                Ok(serde_json::json!({
                    "datadir": self.datadir.to_string_lossy(),
                    "message": result,
                }))
            }
        }
    }
}

//
// Test handlers
//
pub fn handle_list_target() -> anyhow::Result<String, String> {
    Ok(format!(""))
}

pub fn handle_get_target(id: &str) -> anyhow::Result<String, String> {
    Ok(format!("ID: '{id}'"))
}

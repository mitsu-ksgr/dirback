//!
//! # Command dispatcher
//!

use crate::commands::Command;
use crate::commands::CommandType;
use crate::commands::GetTarget;

pub struct Dispatcher {
    pub datadir: std::path::PathBuf,
}

impl Dispatcher {
    pub fn new(datadir: &std::path::Path) -> Self {
        Self {
            datadir: datadir.to_path_buf(),
        }
    }

    pub fn dispatch(&self, cmd: CommandType) -> anyhow::Result<serde_json::Value> {
        match cmd {
            CommandType::ListTarget => {
                let result = String::from("foo");
                Ok(serde_json::json!({ "message": result }))
            }
            CommandType::GetTarget(payload) => {
                let cmd = GetTarget;

                let result = cmd.execute(&self.datadir, payload)?;
                Ok(serde_json::json!(result))
            }
        }
    }
}

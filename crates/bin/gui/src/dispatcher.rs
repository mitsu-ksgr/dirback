//!
//! # Command dispatcher
//!

use crate::commands::GetTarget;
use crate::commands::ListTargets;
use crate::commands::RegisterTarget;
use crate::commands::{Command, CommandType, NoPayload};

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
            CommandType::ListTargets(_) => {
                let cmd = ListTargets;
                let result = cmd.execute(&self.datadir, NoPayload)?;
                Ok(serde_json::json!(result))
            }

            CommandType::GetTarget(payload) => {
                let cmd = GetTarget;
                let result = cmd.execute(&self.datadir, payload)?;
                Ok(serde_json::json!(result))
            }

            CommandType::RegisterTarget(payload) => {
                let cmd = RegisterTarget;
                let result = cmd.execute(&self.datadir, payload)?;
                Ok(serde_json::json!(result))
            }
        }
    }
}

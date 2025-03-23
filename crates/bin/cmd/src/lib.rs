//!
//! # dirback cmd lib
//!

use dirback::infra::app_path;
use std::collections::HashMap;
use std::path::PathBuf;

//-----------------------------------------------------------------------------
//  Args
//-----------------------------------------------------------------------------
pub struct CmdParams {
    pub command: String,
    pub args: Vec<String>,
    pub basedir: PathBuf,
}

impl CmdParams {
    pub fn build(args: &[String]) -> anyhow::Result<Self> {
        if args.len() == 1 {
            anyhow::bail!("No command specified.");
        }

        // if DIRBACK_STORE_DIR is set, use it's value.
        // if not set, use app_path::data_dir.
        let basedir = std::env::var("DIRBACK_STORE_DIR")
            .ok()
            .map(std::path::PathBuf::from)
            .or_else(app_path::data_dir)
            .ok_or_else(|| {
                anyhow::anyhow!("Failed to get path to directory for application data.")
            })?;

        Ok(Self {
            command: args[1].to_string(),
            args: args[2..].to_vec(),
            basedir,
        })
    }
}

//-----------------------------------------------------------------------------
//  Commands
//-----------------------------------------------------------------------------
pub trait Command {
    fn execute(&self, params: &CmdParams) -> anyhow::Result<()>;
}

#[derive(Default)]
pub struct CommandInvoker {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandInvoker {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, command: Box<dyn Command>) {
        self.commands.insert(name.to_string(), command);
    }

    pub fn execute(&self, params: &CmdParams) -> anyhow::Result<()> {
        match self.commands.get(&params.command) {
            Some(cmd) => cmd.execute(params),
            None => anyhow::bail!("Unknown command: '{}'", params.command),
        }
    }
}

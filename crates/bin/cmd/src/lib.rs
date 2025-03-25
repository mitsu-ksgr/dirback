//!
//! # dirback cmd lib
//!

use std::collections::HashMap;
use std::path::{Path, PathBuf};

//-----------------------------------------------------------------------------
//  Args
//-----------------------------------------------------------------------------
pub struct CmdParams {
    pub command: String,
    pub args: Vec<String>,
    pub basedir: PathBuf,
}

impl CmdParams {
    pub fn build(args: &[String], basedir: &Path) -> anyhow::Result<Self> {
        if args.len() == 1 {
            anyhow::bail!("No command specified.");
        }

        Ok(Self {
            command: args[1].to_string(),
            args: args[2..].to_vec(),
            basedir: basedir.to_path_buf(),
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

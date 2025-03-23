//!
//! # dirback cmd lib
//!

use std::collections::HashMap;

pub trait Command {
    fn execute(&self, args: Vec<String>) -> anyhow::Result<()>;
}

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

    pub fn execute(&self, command_name: &str, args: Vec<String>) -> anyhow::Result<()> {
        match self.commands.get(command_name) {
            Some(cmd) => cmd.execute(args),
            None => anyhow::bail!("Unknown command: {}", command_name),
        }
    }
}

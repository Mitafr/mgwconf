use log::{info, warn};

use crate::command::registry::AVAILABLE_COMMANDS;

mod registry;

#[derive(Debug, Clone)]
pub struct Command {
    options: Vec<String>,
    cmd: fn() -> bool,
}

impl Command {
    pub fn new() -> Command {
        Command { options: vec![], cmd: || true }
    }

    pub fn run(&self) -> bool {
        if !self.options.iter().any(|s| AVAILABLE_COMMANDS.contains(&s.as_str())) {
            warn!("Command not recognized {:?}, skipping", self.options);
            info!("Available commands are : {}", AVAILABLE_COMMANDS.join("|"));
            return false;
        }
        info!("Running {:?}", self.options);
        (self.cmd)()
    }
}

impl Default for Command {
    fn default() -> Self {
        Command { options: Vec::new(), cmd: || true }
    }
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        Self { options: vec![value], cmd: || true }
    }
}

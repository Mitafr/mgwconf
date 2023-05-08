use lazy_static::lazy_static;
use log::{info, warn};

lazy_static! {
    static ref AVAILABLE_COMMANDS: [&'static str; 2] = ["GET-SAG", "GET-CERTIFICATES"];
}

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

impl From<String> for Command {
    fn from(value: String) -> Self {
        Self { options: vec![value], cmd: || true }
    }
}

use lazy_static::lazy_static;
use log::{info, warn};

use crate::app::CliApp;

use super::{get_all::GetAll, get_certificate::GetCertificate, get_sag::GetSag, CommandRegistryTrait, CommandTrait};

lazy_static! {
    pub static ref AVAILABLE_COMMANDS: [&'static str; 3] = ["GET-SAG", "GET-CERTIFICATES", "GET-ALL"];
}

pub enum CommandVariant {
    GetAll(GetAll),
    GetSag(GetSag),
    GetCertificate(GetCertificate),
    Unknown,
}

impl CommandRegistryTrait for CommandVariant {
    fn execute(&self, mut app: CliApp) -> std::pin::Pin<Box<dyn std::future::Future<Output = usize> + Send>> {
        match self {
            CommandVariant::GetAll(_cmd) => Box::pin(async move {
                GetAll::execute(&mut app).await;
                GetAll::num_op()
            }),
            CommandVariant::GetSag(_cmd) => Box::pin(async move {
                GetSag::execute(&app).await;
                GetSag::num_op()
            }),
            CommandVariant::GetCertificate(_cmd) => Box::pin(async move {
                GetCertificate::execute(&app).await;
                GetCertificate::num_op()
            }),
            CommandVariant::Unknown => Box::pin(async { 0 }),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            CommandVariant::GetAll(_cmd) => "GetAll",
            CommandVariant::GetSag(_cmd) => "GetSag",
            CommandVariant::GetCertificate(_cmd) => "GetCertificate",
            CommandVariant::Unknown => "Unknown",
        }
    }
}

pub struct Registry<'a> {
    app: &'a mut CliApp,
    options: Vec<String>,
    commands: Vec<CommandVariant>,
}

impl<'a> Registry<'a> {
    pub fn new(app: &mut CliApp, options: Vec<String>) -> Registry {
        let options: Vec<String> = options.iter().map(|s| s.to_uppercase()).collect();
        let commands = options
            .iter()
            .map(|o| match &o[..] {
                "GET-ALL" => CommandVariant::GetAll(GetAll {}),
                "GET-SAG" => CommandVariant::GetSag(GetSag {}),
                "GET-CERTIFICATES" => CommandVariant::GetCertificate(GetCertificate {}),
                _ => CommandVariant::Unknown,
            })
            .collect::<Vec<CommandVariant>>();
        Registry { app, commands, options }
    }

    pub async fn run(self) -> bool {
        if !self.options.iter().any(|s| AVAILABLE_COMMANDS.contains(&s.as_str())) {
            warn!("Command not recognized {:?}, skipping", self.options);
            info!("Available commands are : {}", AVAILABLE_COMMANDS.join("|"));
            return false;
        }
        for command in self.commands.into_iter() {
            info!("Running {:?}", command.name());
            self.app.waiting_res += command.execute(self.app.clone()).await;
        }
        true
    }
}

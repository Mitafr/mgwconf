use lazy_static::lazy_static;
use log::{info, warn};

use crate::app::CliApp;

use super::{
    get_all::GetAll, get_business_application::GetBusinessApplication,
    get_certificate::GetCertificate, get_profile::GetProfile, get_proxy::GetProxy, get_sag::GetSag,
    CommandRegistryTrait, CommandTrait,
};

lazy_static! {
    pub static ref AVAILABLE_COMMANDS: [&'static str; 4] = [
        "GET-SAGS",
        "GET-CERTIFICATES",
        "GET-BUSINESS-APPLICATIONS",
        "GET-ALL"
    ];
}

pub enum CommandVariant {
    GetAll(GetAll),
    GetSag(GetSag),
    GetProfile(GetProfile),
    GetBusinessApplication(GetBusinessApplication),
    GetProxy(GetProxy),
    GetCertificate(GetCertificate),
    Unknown,
}

impl CommandRegistryTrait for CommandVariant {
    fn execute(
        &self,
        app: CliApp,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = usize> + Send>> {
        match self {
            CommandVariant::GetAll(_cmd) => Box::pin(async move {
                GetAll::execute(&app).await;
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
            CommandVariant::GetProfile(_cmd) => Box::pin(async move {
                GetProfile::execute(&app).await;
                GetProfile::num_op()
            }),
            CommandVariant::GetBusinessApplication(_cmd) => Box::pin(async move {
                GetBusinessApplication::execute(&app).await;
                GetBusinessApplication::num_op()
            }),
            CommandVariant::GetProxy(_cmd) => Box::pin(async move {
                GetProxy::execute(&app).await;
                GetProxy::num_op()
            }),
            CommandVariant::Unknown => Box::pin(async { 0 }),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            CommandVariant::GetAll(_cmd) => "GetAll",
            CommandVariant::GetSag(_cmd) => "GetSag",
            CommandVariant::GetCertificate(_cmd) => "GetCertificate",
            CommandVariant::GetProfile(_cmd) => "GetProfile",
            CommandVariant::GetBusinessApplication(_cmd) => "GetBusinessApplication",
            CommandVariant::GetProxy(_cmd) => "GetProxy",
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
                "GET-SAGS" => CommandVariant::GetSag(GetSag {}),
                "GET-CERTIFICATES" => CommandVariant::GetCertificate(GetCertificate {}),
                "GET-BUSINESS-APPLICATIONS" => {
                    CommandVariant::GetBusinessApplication(GetBusinessApplication {})
                }
                _ => CommandVariant::Unknown,
            })
            .collect::<Vec<CommandVariant>>();
        Registry {
            app,
            commands,
            options,
        }
    }

    pub async fn run(self) -> bool {
        if !self
            .options
            .iter()
            .any(|s| AVAILABLE_COMMANDS.contains(&s.as_str()))
        {
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

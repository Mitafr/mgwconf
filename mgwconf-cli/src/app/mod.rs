use anyhow::Result;
use async_trait::async_trait;
use core::panic;
use log::error;
use std::{
    io::{stdin, stdout, Write},
    sync::mpsc::Sender,
};

// use mgwconf_common::{config::Config, model::CollectionEntityTrait, AppTrait};
use mgwconf_network::{model::CollectionEntityTrait, AppConfig, AppTrait, IoEvent};
use mgwconf_vault::{SecretType, SecretsVault};

use crate::{commands::Command, config::Config};

#[derive(Debug)]
pub struct CliApp {
    pub config: Option<Config>,
    pub connectivity_test: bool,
    io_tx: Option<Sender<IoEvent>>,
    pub vault: Option<SecretsVault>,

    initialized: bool,
}

impl Default for CliApp {
    fn default() -> Self {
        CliApp {
            config: None,
            connectivity_test: false,
            io_tx: None,
            vault: None,
            initialized: false,
        }
    }
}

impl CliApp {
    pub async fn new(io_tx: Sender<IoEvent>, mut config: Config, vault_key: &str) -> CliApp {
        config.init_logging();
        let vault = match SecretsVault::new(vault_key) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Can't decode secret vault : {}", e);
                panic!("Can't decode secret vault");
            }
        };
        CliApp {
            config: Some(config),
            io_tx: Some(io_tx),
            vault: Some(vault),
            initialized: false,
            ..Default::default()
        }
    }

    pub async fn run_command(&self, command: Command) {
        if !AppTrait::<Config>::is_connected(self) {
            error!("App is not connected, {:?} is aborted", command);
            return;
        }
        for command in self.config.as_ref().unwrap().commands.iter() {
            if !command.run() {
                error!("Command failed");
            }
        }
    }
}

#[async_trait]
impl<C> AppTrait<C> for CliApp
where
    C: AppConfig,
{
    async fn init(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }
        self.io_tx.as_ref().unwrap().send(IoEvent::Ping)?;
        self.initialized = true;
        Ok(())
    }

    fn dispatch(&mut self, io_event: IoEvent) -> Result<()> {
        self.io_tx.as_ref().unwrap().send(io_event)?;
        Ok(())
    }

    fn ask_secrets(&mut self) -> Result<()> {
        let mut secret = String::new();
        for s in SecretType::iterator() {
            <CliApp as AppTrait<C>>::ask_secret(self, &mut secret, *s);
        }
        print!("\x1B[2J\x1B[1;1H");
        Ok(())
    }

    fn ask_secret(&mut self, s: &mut String, stype: SecretType) {
        println!("Pleaser enter {} API KEY", stype);
        let _ = stdout().flush();
        stdin().read_line(s).expect("Did not enter a correct string");
        s.pop();
        self.vault.as_ref().unwrap().create_secret(stype, s.to_owned());
        s.clear()
    }

    fn is_connected(&self) -> bool {
        self.connectivity_test
    }

    fn set_connected(&mut self, connected: bool) {
        self.connectivity_test = connected;
    }

    fn vault(&self) -> Option<&SecretsVault> {
        self.vault.as_ref()
    }

    fn config(&self) -> Box<dyn AppConfig> {
        Box::new(self.config.as_ref().unwrap().clone())
    }

    fn handle_network_response<T>(&mut self, event: IoEvent, _res: T)
    where
        T: CollectionEntityTrait,
    {
        match event {
            IoEvent::Ping => todo!(),
            IoEvent::GetAllBusinessApplications => todo!(),
            IoEvent::GetAllCertificates => todo!(),
            IoEvent::GetAllSags => todo!(),
            IoEvent::PostBusinessApplication => todo!(),
            IoEvent::PostCertificate => todo!(),
            IoEvent::PostSag => todo!(),
            IoEvent::DeleteBusinessApplication => todo!(),
            IoEvent::DeleteCertificate => todo!(),
            IoEvent::DeleteSag => todo!(),
        }
    }
}

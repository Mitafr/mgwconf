use anyhow::{bail, Error, Result};
use async_trait::async_trait;
use core::panic;
use log::{debug, error};
use mgwconf_network::{event::IoEvent, mgw_configuration::apis::ResponseContent, AppTrait};
use mgwconf_vault::{SecretType, SecretsVault};
use serde::{Deserialize, Serialize};
use std::{io::Write, sync::Arc, time::Duration};
use tokio::sync::{broadcast::Sender, Mutex, Notify};

use crate::{
    command::{
        get_business_application::GetBusinessApplication, get_certificate::GetCertificate,
        get_profile::GetProfile, get_proxy::GetProxy, get_sag::GetSag, registry::Registry,
    },
    config::Config,
    playbook::{error::PlaybookError, Playbook},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum CliAppType {
    Command,
    Playbook,
}

#[derive(Debug, Clone)]
pub struct CliApp {
    pub config: Option<Config>,
    pub connectivity_test: bool,
    io_tx: Sender<IoEvent>,
    pub vault: Option<SecretsVault>,

    initialized: bool,
    pub waiting_res: usize,
    error: bool,
    app_type: CliAppType,
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
        let app_type = if config.playbook.is_some() {
            CliAppType::Playbook
        } else {
            CliAppType::Command
        };
        CliApp {
            config: Some(config),
            io_tx,
            vault: Some(vault),
            initialized: false,
            connectivity_test: false,
            waiting_res: 0,
            error: false,
            app_type,
        }
    }

    pub async fn run_commands(&mut self) {
        if !AppTrait::<Config>::is_connected(self) {
            error!("App is not connected, cli is aborted");
            return;
        }
        let config = self.config.clone();
        let registry = Registry::new(self, config.unwrap().commands.to_owned());
        if !registry.run().await {
            error!("Command failed");
        }
    }

    fn clear_output_dir() {
        log::info!("Clearing output dir");
        for entry in std::fs::read_dir("./output").unwrap().flatten() {
            std::fs::remove_file(entry.path()).unwrap();
        }
    }

    async fn run_playbook(&mut self, playbook: Playbook) -> Result<(), PlaybookError> {
        self.waiting_res = playbook.process(self).await?;
        Ok(())
    }
}

#[async_trait]
impl AppTrait<Config> for CliApp {
    async fn init(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }
        self.io_tx.send(IoEvent::Ping)?;
        self.initialized = true;
        Ok(())
    }

    async fn dispatch(&self, io_event: IoEvent) -> Result<()> {
        self.io_tx.send(io_event)?;
        Ok(())
    }

    fn ask_secrets(master: &str) -> Result<()> {
        let mut secret = String::new();
        for s in SecretType::iterator() {
            <CliApp as AppTrait<Config>>::ask_secret(master, &mut secret, *s);
        }
        print!("\x1B[2J\x1B[1;1H");
        Ok(())
    }

    #[cfg(not(feature = "store"))]
    #[allow(unused_variables)]
    fn ask_secret(master: &str, s: &mut String, stype: SecretType) {
        panic!("To create a vault store you must enable store features");
    }

    #[cfg(feature = "store")]
    fn ask_secret(master: &str, s: &mut String, stype: SecretType) {
        use std::io::{stdin, stdout};
        println!("Pleaser enter {} API KEY", stype);
        let _ = stdout().flush();
        stdin()
            .read_line(s)
            .expect("Did not enter a correct string");
        s.pop();
        SecretsVault::new(master)
            .unwrap()
            .create_secret(stype, s.to_owned())
            .unwrap();
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

    fn config(&self) -> &Config {
        self.config.as_ref().unwrap()
    }

    fn handle_network_response<'a, T: Deserialize<'a> + Serialize>(
        &mut self,
        event: IoEvent,
        res: ResponseContent<T>,
    ) {
        debug!("Receiving response from network for io_event {event:?}");
        match event {
            IoEvent::GetAllForwardProxyEntity => writeln!(
                GetProxy::output_file(),
                "{}",
                serde_json::to_string_pretty(&res.entity).unwrap()
            )
            .unwrap(),
            IoEvent::GetAllBusinessApplications => writeln!(
                GetBusinessApplication::output_file(),
                "{}",
                serde_json::to_string_pretty(&res.entity).unwrap()
            )
            .unwrap(),
            IoEvent::GetAllCertificates => writeln!(
                GetCertificate::output_file(),
                "{}",
                serde_json::to_string_pretty(&res.entity).unwrap()
            )
            .unwrap(),
            IoEvent::GetAllSags => writeln!(
                GetSag::output_file(),
                "{}",
                serde_json::to_string_pretty(&res.entity).unwrap()
            )
            .unwrap(),
            IoEvent::GetAllProfiles => writeln!(
                GetProfile::output_file(),
                "{}",
                serde_json::to_string_pretty(&res.entity).unwrap()
            )
            .unwrap(),
            _ => {}
        }
        self.waiting_res -= 1;
    }

    fn handle_network_error(&mut self, error: Error) {
        log::error!("{}", error);
        if self.app_type != CliAppType::Playbook {
            self.error = true;
        }
        self.waiting_res -= 1;
    }

    async fn run(
        app: Arc<Mutex<Self>>,
        notifier: Option<Arc<Notify>>,
    ) -> Result<(), anyhow::Error> {
        <CliApp as AppTrait<Config>>::init(&mut *app.lock().await).await?;
        log::info!("Waiting for Network");
        notifier.unwrap().notified().await;
        if !AppTrait::<Config>::is_connected(&*app.lock().await) {
            bail!("Network has not been initialized correctly");
        }
        log::info!("Network initialized, running command");
        {
            let playbook = {
                let app = &mut *app.lock().await;
                AppTrait::<Config>::config(app).playbook.clone()
            };
            let app = &mut *app.lock().await;
            if let Some(playbook) = playbook {
                app.run_playbook(playbook).await?
            } else {
                Self::clear_output_dir();
                let run_command = app.run_commands();
                run_command.await;
            }
        }
        'main: loop {
            {
                let app = &mut *app.lock().await;
                if app.error {
                    bail!("An error occured");
                }
                if app.waiting_res == 0 {
                    break 'main;
                }
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        Ok(())
    }
}

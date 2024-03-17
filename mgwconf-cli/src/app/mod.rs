use anyhow::{bail, Error, Result};
use async_trait::async_trait;
use core::panic;
use log::{debug, error};
use mgwconf_network::{event::IoEvent, AppConfig, AppTrait};
use mgwconf_vault::{SecretType, SecretsVault};
use std::{
    io::{stdin, stdout, Write},
    sync::Arc,
    time::Duration,
};
use tokio::sync::{mpsc::Sender, Mutex, Notify};

use crate::{
    command::{get_sag::GetSag, registry::Registry},
    config::Config,
};

#[derive(Debug, Clone)]
pub struct CliApp {
    pub config: Option<Config>,
    pub connectivity_test: bool,
    io_tx: Sender<IoEvent>,
    pub vault: Option<SecretsVault>,

    initialized: bool,
    pub waiting_res: usize,
    error: bool,
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
            io_tx,
            vault: Some(vault),
            initialized: false,
            connectivity_test: false,
            waiting_res: 0,
            error: false,
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
        for entry in std::fs::read_dir("./output").unwrap() {
            if let Ok(entry) = entry {
                std::fs::remove_file(entry.path()).unwrap();
            }
        }
    }

    fn config<'a>(&'a self) -> &'a Option<Config> {
        &self.config
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
        self.io_tx.send(IoEvent::Ping).await?;
        self.initialized = true;
        Ok(())
    }

    async fn dispatch(&self, io_event: IoEvent) -> Result<()> {
        self.io_tx.send(io_event).await?;
        Ok(())
    }

    fn ask_secrets(master: &str) -> Result<()> {
        let mut secret = String::new();
        for s in SecretType::iterator() {
            <CliApp as AppTrait<C>>::ask_secret(master, &mut secret, *s);
        }
        print!("\x1B[2J\x1B[1;1H");
        Ok(())
    }

    fn ask_secret(master: &str, s: &mut String, stype: SecretType) {
        println!("Pleaser enter {} API KEY", stype);
        let _ = stdout().flush();
        stdin().read_line(s).expect("Did not enter a correct string");
        s.pop();
        let vault = SecretsVault::new(master).unwrap();
        vault.create_secret(stype, s.to_owned()).unwrap();
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

    fn handle_network_response(&mut self, event: IoEvent, res: serde_json::Value) {
        debug!("Receiving response from network for io_event {event:?}");
        match event {
            IoEvent::GetAllForwardProxyEntity => writeln!(GetSag::output_file(), "GetAllForwardProxyEntity: {}", serde_json::to_string_pretty(&res).unwrap()).unwrap(),
            IoEvent::GetAllBusinessApplications => writeln!(GetSag::output_file(), "GetAllBusinessApplications: {}", serde_json::to_string_pretty(&res).unwrap()).unwrap(),
            IoEvent::GetAllApplicationProfileEntity => writeln!(GetSag::output_file(), "GetAllApplicationProfileEntity: {}", serde_json::to_string_pretty(&res).unwrap()).unwrap(),
            IoEvent::GetAllCertificates => writeln!(GetSag::output_file(), "GetAllCertificates: {}", serde_json::to_string_pretty(&res).unwrap()).unwrap(),
            IoEvent::GetAllSags => writeln!(GetSag::output_file(), "GetSags: {}", serde_json::to_string_pretty(&res).unwrap()).unwrap(),
            IoEvent::GetAllProfiles => writeln!(GetSag::output_file(), "GetAllProfiles: {}", serde_json::to_string_pretty(&res).unwrap()).unwrap(),
            _ => todo!(),
        }
        self.waiting_res -= 1;
    }

    fn handle_network_error(&mut self, error: Error) {
        log::error!("{}", error);
        self.error = true;
    }

    async fn run(app: Arc<Mutex<Self>>, notifier: Option<Arc<Notify>>) -> Result<(), anyhow::Error> {
        <CliApp as AppTrait<C>>::init(&mut *app.lock().await).await?;
        log::info!("Waiting for Network");
        notifier.unwrap().notified().await;
        if !AppTrait::<C>::is_connected(&*app.lock().await) {
            bail!("Network has not been initialized correctly");
        }
        log::info!("Network initialized, running command");
        {
            let app = &mut *app.lock().await;
            match <CliApp>::config(app) {
                Some(config) => {
                    Self::clear_output_dir();
                    if let Some(playbook) = &config.playbook {
                        playbook.process(app).await?;
                        app.waiting_res += 1;
                    } else {
                        let run_command = app.run_commands();
                        run_command.await;
                    }
                }
                None => {}
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

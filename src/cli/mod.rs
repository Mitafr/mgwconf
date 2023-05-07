use std::sync::Arc;

use mgwconf_cli::app::CliApp;
use mgwconf_network::AppConfig;
use mgwconf_network::AppTrait;
use std::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

use clap::Parser;
use mgwconf_cli::config::Config;
use mgwconf_network::IoEvent;

use mgwconf_cli::commands::Command;

pub async fn start_cli<C: AppConfig>(app: Arc<Mutex<CliApp>>, pair: Arc<Notify>) -> Result<(), anyhow::Error> {
    <CliApp as AppTrait<C>>::init(&mut *app.lock().await).await?;
    log::info!("Waiting for Network");
    pair.notified().await;
    log::info!("Network initialized, running command");
    app.lock().await.run_command(Command::new()).await;
    Ok(())
}

pub async fn create_app(io_tx: Sender<IoEvent>) -> (Arc<Mutex<CliApp>>, Config) {
    use mgwconf_cli::config::Args;

    use crate::ask_master_key;

    let args = Args::parse();
    let vault_key = if args.vault_key.is_some() { args.vault_key.as_ref().unwrap().to_owned() } else { ask_master_key() };

    let config = Config::init(&args).unwrap();
    (Arc::new(Mutex::new(CliApp::new(io_tx, config.clone(), &vault_key).await)), config)
}

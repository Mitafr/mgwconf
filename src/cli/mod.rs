use std::sync::Arc;

use mgwconf_cli::app::CliApp;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

use clap::Parser;
use mgwconf_cli::config::Config;
use mgwconf_network::event::IoEvent;

pub async fn create_app(io_tx: Sender<IoEvent>) -> (Arc<Mutex<CliApp>>, Config) {
    use mgwconf_cli::config::Args;

    use crate::ask_master_key;

    let args = Args::parse();
    let vault_key = if args.vault_key.is_some() { args.vault_key.as_ref().unwrap().to_owned() } else { ask_master_key() };

    let config = Config::init(&args).unwrap();
    (Arc::new(Mutex::new(CliApp::new(io_tx, config.clone(), &vault_key).await)), config)
}

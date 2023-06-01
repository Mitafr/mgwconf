use std::sync::Arc;
use tokio::sync::Mutex;

use mgwconf_network::event::IoEvent;
use tokio::sync::mpsc::Sender;

pub use mgwconf_ui::{
    app::{UiApp, UiAppTrait},
    event::Key,
};

use mgwconf_ui::config::{Args, Config};

pub async fn create_app(io_tx: Sender<IoEvent>) -> (Arc<Mutex<UiApp>>, Config) {
    use crate::ask_master_key;
    use clap::Parser;

    let args = Args::parse();
    let vault_key = if args.vault_key.is_some() { args.vault_key.as_ref().unwrap().to_owned() } else { ask_master_key() };

    let config = Config::init(&args).unwrap();
    (Arc::new(Mutex::new(UiApp::new(io_tx, config.clone(), &vault_key).await)), config)
}

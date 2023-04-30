#[cfg(feature = "ui")]
use crate::ui::UiApp;
use clap::Parser;
#[cfg(not(feature = "ui"))]
use cli::start_cli;
use log::{error, info};
#[cfg(not(feature = "ui"))]
use mgwconf_cli::app::CliApp;
use mgwconf_network::{IoEvent, Network};
#[cfg(feature = "ui")]
use ui::start_ui;

use anyhow::Result;
use mgwconf_common::{
    config::{Args, Config},
    AppTrait,
};
use std::{
    io::{stdin, stdout, Write},
    panic,
    sync::Arc,
};
use tokio::sync::{Mutex, Notify};

#[cfg(not(feature = "ui"))]
mod cli;
#[cfg(feature = "ui")]
mod ui;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut config = Config::init(&args).expect("Could not init config");
    config.init_logging();

    let vault_key = if args.vault_key.is_some() { args.vault_key.unwrap() } else { ask_master_key() };
    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IoEvent>();
    #[cfg(not(feature = "ui"))]
    let app = Arc::new(Mutex::new(CliApp::new(sync_io_tx, config.clone(), &vault_key).await));
    #[cfg(feature = "ui")]
    let app = Arc::new(Mutex::new(UiApp::new(sync_io_tx, config.clone(), &vault_key).await));
    let cloned_app = Arc::clone(&app);

    let orig = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        error!("{}", panic_info);
        orig(panic_info);
        std::process::exit(1);
    }));
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();
    std::thread::spawn(move || {
        let mut net = Network::new(&app, &config).expect("Network Error");
        start_tokio(sync_io_rx, &mut net, notify2);
    });
    cloned_app.lock().await.vault.as_mut().expect("Vault not initialized correctly").read_all_secrets();
    #[cfg(feature = "ui")]
    {
        start_ui(&cloned_app).await.unwrap();
    }
    #[cfg(not(feature = "ui"))]
    {
        start_cli(&cloned_app, notify).await.unwrap();
    }
    Ok(())
}

#[tokio::main]
async fn start_tokio<A: AppTrait>(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network<A>, pair2: Arc<Notify>) {
    info!("Notifying thread");
    while let Ok(io_event) = io_rx.recv() {
        match network.handle_network_event(io_event).await {
            Ok(_) => {
                pair2.notify_one();
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }
    }
}

fn ask_master_key() -> String {
    let mut vault_key = String::new();
    println!("Pleaser enter MASTER VAULT KEY");
    let _ = stdout().flush();
    stdin().read_line(&mut vault_key).expect("Did not enter a correct string");
    vault_key.pop();
    print!("\x1B[2J\x1B[1;1H");
    vault_key
}

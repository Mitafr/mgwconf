use log::{error, info};
use mgwconf_network::{event::IoEvent, AppConfig, AppTrait, Network};

use std::sync::Arc;
use tokio::sync::{mpsc::Receiver, Mutex};

use tokio::sync::mpsc::Sender;

pub use mgwconf_ui::{
    app::{UiApp, UiAppTrait},
    event::Key,
};

use mgwconf_ui::config::{Args, Config};

use anyhow::Result;
use std::{
    io::{stdin, stdout, Write},
    panic,
};
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<()> {
    let (sync_io_tx, sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(100);
    let (app, config) = create_app(sync_io_tx).await;
    let cloned_app = Arc::clone(&app);

    let orig = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        error!("{}", panic_info);
        orig(panic_info);
        std::process::exit(1);
    }));
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();
    cloned_app.lock().await.vault.as_mut().expect("Vault not initialized correctly").read_all_secrets();
    std::thread::spawn(move || {
        let mut net = Network::new(&app, &config).expect("Network Error");
        start_tokio(sync_io_rx, &mut net, notify2);
    });
    use mgwconf_ui::app::UiApp;
    use mgwconf_ui::config::Config;
    <UiApp as AppTrait<Config>>::run(cloned_app, None).await.unwrap();
    info!("Exiting");
    Ok(())
}

pub async fn create_app(io_tx: Sender<IoEvent>) -> (Arc<Mutex<UiApp>>, Config) {
    use clap::Parser;

    let args = Args::parse();
    let vault_key = if args.vault_key.is_some() { args.vault_key.as_ref().unwrap().to_owned() } else { ask_master_key() };
    let mut config = Config::init(&args).unwrap();
    config.init_logging();
    if args.create_secret {
        <UiApp as AppTrait<Config>>::ask_secrets(&vault_key).unwrap();
    }

    (Arc::new(Mutex::new(UiApp::new(io_tx, config.clone(), &vault_key).await)), config)
}

#[tokio::main]
async fn start_tokio<A: AppTrait<C>, C: AppConfig>(mut io_rx: Receiver<IoEvent>, network: &mut Network<A, C>, pair2: Arc<Notify>) {
    info!("Notifying thread");
    while let Some(io_event) = io_rx.recv().await {
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

pub fn ask_master_key() -> String {
    let mut vault_key = String::new();
    println!("Pleaser enter MASTER VAULT KEY");
    let _ = stdout().flush();
    stdin().read_line(&mut vault_key).expect("Did not enter a correct string");
    vault_key.pop();
    print!("\x1B[2J\x1B[1;1H");
    vault_key
}

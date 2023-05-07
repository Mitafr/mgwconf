use log::{error, info};
use mgwconf_network::{AppConfig, AppTrait, IoEvent, Network};

use anyhow::Result;
use std::{
    io::{stdin, stdout, Write},
    panic,
    sync::Arc,
};
use tokio::sync::Notify;

#[cfg(not(feature = "ui"))]
mod cli;
#[cfg(feature = "ui")]
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    // let args = Args::parse();
    // let mut config = Config::init(&args).expect("Could not init config");
    // config.init_logging();

    // let vault_key = if args.vault_key.is_some() { args.vault_key.unwrap() } else { ask_master_key() };
    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IoEvent>();
    #[cfg(not(feature = "ui"))]
    let (app, config) = cli::create_app(sync_io_tx).await;
    #[cfg(feature = "ui")]
    let (app, config) = ui::create_app(sync_io_tx).await;
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
        use mgwconf_ui::config::Config;
        ui::start_ui::<Config>(cloned_app).await.unwrap();
    }
    #[cfg(not(feature = "ui"))]
    {
        use mgwconf_cli::config::Config;
        cli::start_cli::<Config>(cloned_app, notify).await.unwrap();
    }
    info!("Exiting");
    Ok(())
}

#[tokio::main]
async fn start_tokio<A: AppTrait<C>, C: AppConfig>(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network<A, C>, pair2: Arc<Notify>) {
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

pub fn ask_master_key() -> String {
    let mut vault_key = String::new();
    println!("Pleaser enter MASTER VAULT KEY");
    let _ = stdout().flush();
    stdin().read_line(&mut vault_key).expect("Did not enter a correct string");
    vault_key.pop();
    print!("\x1B[2J\x1B[1;1H");
    vault_key
}

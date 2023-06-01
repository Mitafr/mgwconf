use log::{error, info};
use mgwconf_network::{event::IoEvent, AppConfig, AppTrait, Network};

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
    let (sync_io_tx, sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(100);
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
        use mgwconf_ui::app::UiApp;
        use mgwconf_ui::config::Config;
        <UiApp as AppTrait<Config>>::run(cloned_app, None).await.unwrap();
    }
    #[cfg(not(feature = "ui"))]
    {
        use mgwconf_cli::app::CliApp;
        use mgwconf_cli::config::Config;
        <CliApp as AppTrait<Config>>::run(cloned_app, Some(notify)).await?;
    }
    info!("Exiting");
    Ok(())
}

#[tokio::main]
async fn start_tokio<A: AppTrait<C>, C: AppConfig>(mut io_rx: tokio::sync::mpsc::Receiver<IoEvent>, network: &mut Network<A, C>, pair2: Arc<Notify>) {
    info!("Notifying thread");
    loop {
        if let Ok(io_event) = io_rx.try_recv() {
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

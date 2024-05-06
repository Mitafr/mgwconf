use std::{
    io::{stdin, stdout, Write},
    panic,
    sync::Arc,
    time::Instant,
};

use clap::Parser;
use log::{error, info};
use tokio::sync::{
    broadcast::{channel, Receiver, Sender},
    Mutex, Notify,
};

use mgwconf_cli::{
    app::CliApp,
    config::{Args, Config},
};
use mgwconf_network::{event::IoEvent, AppConfig, AppTrait, Network};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let (sync_io_tx, sync_io_rx) = channel(100);
    let (app, config) = create_app(sync_io_tx).await;
    let cloned_app = Arc::clone(&app);

    let orig = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        error!("{}", panic_info);
        orig(panic_info);
        std::process::exit(1);
    }));
    let notify = Arc::new(Notify::new());
    log::info!("Reading secrets from vault");
    cloned_app
        .lock()
        .await
        .vault
        .as_mut()
        .expect("Vault not initialized correctly")
        .read_all_secrets();
    let now = Instant::now();
    log::info!("Starting Network");
    if let Some(ref playbook) = config.playbook {
        for host in playbook.entries.hosts.iter() {
            let notify2 = notify.clone();
            let sync_io_rx = sync_io_rx.resubscribe();
            let app = app.clone();
            let mut config = config.clone();
            config.remote_addr = *host;
            std::thread::spawn(move || {
                let mut net = Network::new(&app, &config).expect("Network Error");
                start_tokio(sync_io_rx, &mut net, notify2);
            });
        }
    } else {
        let notify2 = notify.clone();
        std::thread::spawn(move || {
            let mut net = Network::new(&app, &config).expect("Network Error");
            start_tokio(sync_io_rx, &mut net, notify2);
        });
    }
    match <CliApp as AppTrait<Config>>::run(cloned_app, Some(notify)).await {
        Ok(_) => {
            info!("Elapsed time : {:.9}s", now.elapsed().as_secs_f64(),);
            Ok(())
        }
        Err(e) => {
            error!("{}, exiting", e);
            Err(e)
        }
    }
}

pub async fn create_app(io_tx: Sender<IoEvent>) -> (Arc<Mutex<CliApp>>, Config) {
    let args = Args::parse();
    let vault_key = if args.vault_key.is_some() {
        args.vault_key.as_ref().unwrap().to_owned()
    } else {
        ask_master_key()
    };
    let mut config = Config::init(&args).unwrap();
    config.init_logging();
    if args.create_secret {
        <CliApp as AppTrait<Config>>::ask_secrets(&vault_key).unwrap();
    }
    (
        Arc::new(Mutex::new(
            CliApp::new(io_tx, config.clone(), &vault_key).await,
        )),
        config,
    )
}

#[tokio::main]
async fn start_tokio<A: AppTrait<C>, C: AppConfig>(
    mut io_rx: Receiver<IoEvent>,
    network: &mut Network<A, C>,
    pair2: Arc<Notify>,
) {
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
    stdin()
        .read_line(&mut vault_key)
        .expect("Did not enter a correct string");
    vault_key.pop();
    print!("\x1B[2J\x1B[1;1H");
    vault_key
}

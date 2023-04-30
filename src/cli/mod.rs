use std::sync::Arc;

use mgwconf_cli::app::CliApp;
use mgwconf_common::AppTrait;
use tokio::sync::{Mutex, Notify};

use mgwconf_cli::commands::Command;

pub async fn start_cli(app: &Arc<Mutex<CliApp>>, pair: Arc<Notify>) -> Result<(), anyhow::Error> {
    app.lock().await.init().await?;
    log::info!("Waiting for Network");
    pair.notified().await;
    log::info!("Network initialized, running command");
    app.lock().await.run_command(Command::new()).await;
    Ok(())
}

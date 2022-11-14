use reqwest::header::{HeaderMap, HeaderValue};
use tokio::sync::MutexGuard;

use crate::{app::vault::SecretType, config::Config, ui::prelude::App};

pub fn generate_api_header(app: &MutexGuard<'_, App>, stype: SecretType) -> HeaderMap {
    let mut header = HeaderMap::new();
    header.append(
        "X-API-KEY",
        HeaderValue::from_str(app.vault.as_ref().expect("Vault not initialized correctly").get_secret(stype).as_ref().unwrap()).unwrap(),
    );
    header
}

pub fn route_url(config: &Config, path: &str) -> String {
    format!("https://{}:{}/swift/mgw/{}", config.remote_ip, config.remote_port, path)
}

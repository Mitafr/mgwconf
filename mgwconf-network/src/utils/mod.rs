use mgwconf_vault::SecretType;
use reqwest::header::{HeaderMap, HeaderValue};
use tokio::sync::MutexGuard;

use crate::{AppConfig, AppTrait};

pub fn generate_api_header<A: AppTrait<C>, C: AppConfig>(app: &MutexGuard<'_, A>, stype: SecretType) -> HeaderMap {
    let mut header = HeaderMap::new();
    header.append(
        "X-API-KEY",
        HeaderValue::from_str(app.vault().as_ref().expect("Vault not initialized correctly").get_secret(stype).as_ref().unwrap()).unwrap(),
    );
    header
}

pub fn route_url<C>(config: &C, path: &str) -> String
where
    C: AppConfig,
{
    format!("https://{}:{}/swift/mgw/{}", config.remote_ip(), config.remote_port(), path)
}

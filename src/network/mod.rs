use std::{fs::File, io::Read, sync::Arc};

use anyhow::Result;
use log::info;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Certificate, Client, Response, StatusCode,
};
use tokio::sync::Mutex;

use crate::{app::App, config::Config};

#[derive(Debug)]
pub enum IoEvent {
    Ping,
    GetAllSags,
}

#[derive(Clone)]
pub struct Network<'a> {
    pub app: &'a Arc<Mutex<App>>,
    client: Client,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>, config: &Config) -> Result<Self> {
        let certificate = get_mgw_root_cert(config)?;
        let client = reqwest::Client::builder().local_address(config.remote_ip).add_root_certificate(certificate).build()?;
        Ok(Network { app, client })
    }

    pub async fn ping_mgw(&mut self) -> Result<Option<Response>, reqwest::Error> {
        let res = self.client.get("https://localhost:9003/swift/mgw/mgw-monitoring-api/1.0.0/health").send().await?;
        let mut app = self.app.lock().await;
        app.connectivity_test = true;
        Ok(Some(res))
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) -> Result<Option<Response>> {
        match io_event {
            IoEvent::Ping => self.ping_mgw().await?,
            IoEvent::GetAllSags => self.get_all_sags().await?,
        };
        Ok(None)
    }

    pub async fn get_all_sags(&mut self) -> Result<Option<Response>> {
        let app = self.app.lock().await;
        let mut header = HeaderMap::new();
        header.append(
            "X-API-KEY",
            HeaderValue::from_str(&app.vault.as_ref().expect("Vault not initialized correctly").configuration.as_ref().unwrap()).unwrap(),
        );
        let res = self.client.get("https://localhost:9003/swift/mgw/mgw-configuration-api/2.0.0/sag").headers(header).send().await?;
        if ![StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
            return Err(anyhow::Error::msg(format!("{:?}", res)));
        }
        info!("{:?}", res);
        Ok(Some(res))
    }
}

fn get_mgw_root_cert(config: &Config) -> anyhow::Result<Certificate> {
    let mut buf = Vec::new();
    File::open(&config.root_ca_path)?.read_to_end(&mut buf)?;
    Ok(reqwest::Certificate::from_pem(&buf).unwrap())
}

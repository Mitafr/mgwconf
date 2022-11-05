use std::{fs::File, io::Read, sync::Arc};

use reqwest::{Certificate, Client, Response};
use tokio::sync::Mutex;

use crate::{app::App, config::Config};

#[derive(Debug)]
pub enum IoEvent {
    Ping,
}

#[derive(Clone)]
pub struct Network<'a> {
    pub app: &'a Arc<Mutex<App>>,
    client: Client,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>, config: &Config) -> Self {
        let certificate = get_mgw_root_cert(config).unwrap();
        let client = reqwest::Client::builder().local_address(config.remote_ip).add_root_certificate(certificate).build().unwrap();
        Network { app, client }
    }

    pub async fn ping_mgw(&mut self) -> Result<Response, reqwest::Error> {
        let res = self.client.get("https://localhost:9003/swift/mgw/mgw-monitoring-api/1.0.0/health").send().await?;
        let mut app = self.app.lock().await;
        app.connectivity_test = true;
        Ok(res)
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::Ping => self.ping_mgw().await.unwrap(),
        };
    }
}

fn get_mgw_root_cert(config: &Config) -> anyhow::Result<Certificate> {
    let mut buf = Vec::new();
    File::open(&config.root_ca_path)?.read_to_end(&mut buf)?;
    Ok(reqwest::Certificate::from_pem(&buf).unwrap())
}

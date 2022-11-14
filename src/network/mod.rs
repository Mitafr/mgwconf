use std::{fs::File, io::Read, sync::Arc};

use anyhow::Result;
use reqwest::{Certificate, Client, StatusCode};
use tokio::sync::Mutex;

use crate::{app::App, config::Config};

use self::{controller::*, utils::route_url};

pub mod controller;
pub mod model;
pub mod utils;

#[derive(Debug)]
pub enum IoEvent {
    Ping,
    GetAllBusinessApplications,
    GetAllCertificates,
    GetAllSags,
    PostBusinessApplication,
    PostCertificate,
    PostSag,
}

#[derive(Clone)]
pub struct Network<'a> {
    pub app: &'a Arc<Mutex<App>>,
    config: &'a Config,
    client: Client,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>, config: &'a Config) -> Result<Self> {
        let certificate = get_mgw_root_cert(config)?;
        let client = reqwest::Client::builder().local_address(config.remote_ip).add_root_certificate(certificate).build()?;
        Ok(Network { app, client, config })
    }

    pub async fn ping_mgw(&mut self) -> Result<(), anyhow::Error> {
        let res = self.client.get(route_url(self.config, "mgw-monitoring-api/1.0.0/health")).send().await?;
        let mut app = self.app.lock().await;
        if [StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
            app.connectivity_test = false;
        } else {
            app.connectivity_test = true;
        }
        Ok(())
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) -> Result<(), anyhow::Error> {
        match io_event {
            IoEvent::Ping => self.ping_mgw().await?,
            IoEvent::GetAllSags => sag::get_all_sags(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::PostSag => sag::post_sag(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::GetAllCertificates => certificate::get_all_certificates(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::PostCertificate => certificate::post_certificate(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::GetAllBusinessApplications => business_application::get_all_business_applications(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::PostBusinessApplication => business_application::post_business_application(self.app.lock().await, &self.client, self.config).await?,
        };
        Ok(())
    }
}

fn get_mgw_root_cert(config: &Config) -> anyhow::Result<Certificate> {
    let mut buf = Vec::new();
    File::open(&config.root_ca_path)?.read_to_end(&mut buf)?;
    Ok(reqwest::Certificate::from_pem(&buf).unwrap())
}

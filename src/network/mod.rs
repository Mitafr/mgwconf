use std::{fs::File, io::Read, sync::Arc};

use anyhow::Result;
use reqwest::{Certificate, Client, StatusCode};
use tokio::sync::Mutex;

use crate::{app::App, config::Config};

use self::model::ModelTrait;
use self::utils::route_url;

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
    DeleteBusinessApplication,
    DeleteCertificate,
    DeleteSag,
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
            IoEvent::GetAllSags => self.app.lock().await.configuration_state.sags = model::sag::SagEntities::get(self.app.lock().await, &self.client, self.config).await?, //sag::get_all_sags(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::PostSag => model::sag::SagEntities::post(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::DeleteSag => model::sag::SagEntities::delete(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::GetAllCertificates => self.app.lock().await.configuration_state.certificates = model::certificate::CertificateEntities::get(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::PostCertificate => model::certificate::CertificateEntities::post(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::DeleteCertificate => model::certificate::CertificateEntities::delete(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::GetAllBusinessApplications => {
                self.app.lock().await.configuration_state.business_applications = model::business_application::BusinessApplications::get(self.app.lock().await, &self.client, self.config).await?
            }
            IoEvent::PostBusinessApplication => model::business_application::BusinessApplications::post(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::DeleteBusinessApplication => model::business_application::BusinessApplications::delete(self.app.lock().await, &self.client, self.config).await?,
        };
        Ok(())
    }
}

fn get_mgw_root_cert(config: &Config) -> anyhow::Result<Certificate> {
    let mut buf = Vec::new();
    File::open(&config.root_ca_path)?.read_to_end(&mut buf)?;
    Ok(reqwest::Certificate::from_pem(&buf).unwrap())
}

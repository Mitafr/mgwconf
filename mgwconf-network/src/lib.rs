use std::{fs::File, io::Read, net::IpAddr, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use log::{error, info};
use mgwconf_vault::{SecretType, SecretsVault};
use model::CollectionEntityTrait;
use model::ModelTrait;
use reqwest::{Certificate, Client, StatusCode};
use tokio::sync::Mutex;
use utils::route_url;

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

#[async_trait]
pub trait AppConfig: Send + Sync {
    fn remote_ip(&self) -> IpAddr;
    fn remote_port(&self) -> u16;
    fn root_ca_path(&self) -> String;
    fn tickrate(&self) -> u64;
}

#[async_trait]
pub trait AppTrait<C: AppConfig>: Send {
    async fn init(&mut self) -> Result<()>;
    fn dispatch(&mut self, io_event: IoEvent) -> Result<()>;

    fn ask_secrets(&mut self) -> Result<()>;
    fn ask_secret(&mut self, s: &mut String, stype: SecretType);

    fn is_connected(&self) -> bool;
    fn set_connected(&mut self, connected: bool);

    fn vault(&self) -> Option<&SecretsVault>;
    fn config(&self) -> Box<dyn AppConfig>;

    fn handle_network_response<T>(&mut self, event: IoEvent, res: T)
    where
        T: CollectionEntityTrait;
}

#[derive(Clone)]
pub struct Network<'a, A, C>
where
    A: AppTrait<C>,
    C: AppConfig,
{
    pub app: &'a Arc<Mutex<A>>,
    client: Client,
    config: &'a C,
}

impl<'a, A, C> Network<'a, A, C>
where
    A: AppTrait<C>,
    C: AppConfig,
{
    pub fn new(app: &'a Arc<Mutex<A>>, config: &'a C) -> Result<Self> {
        let config = config;
        let certificate = get_mgw_root_cert(config)?;
        let client = reqwest::Client::builder().local_address(config.remote_ip()).add_root_certificate(certificate).build()?;
        Ok(Network { app, client, config })
    }

    pub async fn ping_mgw(&mut self) -> Result<(), anyhow::Error> {
        let route = route_url(self.config, "mgw-monitoring-api/1.0.0/health");
        match self.client.get(route).send().await {
            Ok(res) => {
                let mut app = self.app.lock().await;
                if [StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
                    app.set_connected(false);
                } else {
                    app.set_connected(true);
                }
                info!("Send ping result : {} -> connected : {}", res.status(), app.is_connected());
            }
            Err(e) => {
                error!("{}", e);
            }
        }
        Ok(())
    }

    #[cfg(feature = "ui")]
    pub async fn handle_network_event(&mut self, io_event: IoEvent) -> Result<(), anyhow::Error> {
        match io_event {
            IoEvent::Ping => self.ping_mgw().await?,
            IoEvent::GetAllSags => {
                let entities = model::sag::SagEntities::get_all(self.app.lock().await, &self.client, self.config).await?;
                self.app.lock().await.handle_network_response::<model::sag::Entities>(IoEvent::GetAllSags, entities);
            } //= model::sag::SagEntities::get_all(self.app.lock().await, &self.client, self.config).await?, //sag::get_all_sags(self.app.lock().await, &self.client, self.config).await?,
            // IoEvent::GetAllSags => self.app.lock().await.configuration_state.sags = model::sag::SagEntities::get_all(self.app.lock().await, &self.client, self.config).await?,
            //sag::get_all_sags(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::PostSag => model::sag::SagEntities::post(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::DeleteSag => model::sag::SagEntities::delete(self.app.lock().await, &self.client, self.config).await?,
            IoEvent::GetAllCertificates => {
                let entities = model::certificate::CertificateEntities::get_all(self.app.lock().await, &self.client, self.config).await?;
                self.app.lock().await.handle_network_response::<model::certificate::Entities>(IoEvent::GetAllCertificates, entities);
            }
            // IoEvent::PostCertificate => model::certificate::CertificateEntities::post(self.app.lock().await, &self.client, self.config).await?,
            // IoEvent::DeleteCertificate => model::certificate::CertificateEntities::delete(self.app.lock().await, &self.client, self.config).await?,
            // IoEvent::GetAllBusinessApplications => {
            //     self.app.lock().await.configuration_state.business_applications = model::business_application::BusinessApplications::get_all(self.app.lock().await, &self.client, self.config).await?
            // }
            // IoEvent::PostBusinessApplication => model::business_application::BusinessApplications::post(self.app.lock().await, &self.client, self.config).await?,
            // IoEvent::DeleteBusinessApplication => model::business_application::BusinessApplications::delete(self.app.lock().await, &self.client, self.config).await?,
            _ => {}
        };
        Ok(())
    }

    #[cfg(not(feature = "ui"))]
    pub async fn handle_network_event(&mut self, io_event: IoEvent) -> Result<(), anyhow::Error> {
        match io_event {
            IoEvent::Ping => self.ping_mgw().await?,
            _ => {}
            // IoEvent::GetAllSags => self.app.lock().await.configuration_state.sags = model::sag::SagEntities::get_all(self.app.lock().await, &self.client, self.config).await?, //sag::get_all_sags(self.app.lock().await, &self.client, self.config).await?,
            // IoEvent::PostSag => model::sag::SagEntities::post(self.app.lock().await, &self.client, self.config).await?,
            // IoEvent::DeleteSag => model::sag::SagEntities::delete(self.app.lock().await, &self.client, self.config).await?,
            // IoEvent::GetAllCertificates => self.app.lock().await.configuration_state.certificates = model::certificate::CertificateEntities::get_all(self.app.lock().await, &self.client, self.config).await?,
            // IoEvent::PostCertificate => model::certificate::CertificateEntities::post(self.app.lock().await, &self.client, self.config).await?,
            // IoEvent::DeleteCertificate => model::certificate::CertificateEntities::delete(self.app.lock().await, &self.client, self.config).await?,
            // IoEvent::GetAllBusinessApplications => {
            //     self.app.lock().await.configuration_state.business_applications = model::business_application::BusinessApplications::get_all(self.app.lock().await, &self.client, self.config).await?
            // }
            // IoEvent::PostBusinessApplication => model::business_application::BusinessApplications::post(self.app.lock().await, &self.client, self.config).await?,
            // IoEvent::DeleteBusinessApplication => model::business_application::BusinessApplications::delete(self.app.lock().await, &self.client, self.config).await?,
        };
        Ok(())
    }
}

fn get_mgw_root_cert<T>(config: &T) -> anyhow::Result<Certificate>
where
    T: AppConfig,
{
    let mut buf = Vec::new();
    File::open(&config.root_ca_path())?.read_to_end(&mut buf)?;
    Ok(reqwest::Certificate::from_pem(&buf).unwrap())
}

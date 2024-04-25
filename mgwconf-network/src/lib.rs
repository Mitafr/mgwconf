#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::any::Any;
use std::net::SocketAddr;
use std::time::Duration;
use std::{fs::File, io::Read, net::IpAddr, sync::Arc};

use anyhow::{Error, Result};
use async_trait::async_trait;
use event::IoEvent;
use log::debug;
use log::{error, info};
use mgwconf_vault::{SecretType, SecretsVault};
pub use reqwest::Identity;
use reqwest::{Certificate, Client, StatusCode};
use tokio::sync::Mutex;
use tokio::sync::Notify;

use crate::handler::api_client_credentials::ApiClientCredentialHandler;
use crate::handler::business_application::BusinessApplicationHandler;
use crate::handler::cert::CertHandler;
use crate::handler::forward_proxy::ForwardProxyHandler;
use crate::handler::profile::ProfileHandler;
use crate::handler::{sag::SagHandler, Handler};

pub mod api;
pub mod event;
pub mod handler;
pub mod model;

#[async_trait]
pub trait AppConfig: Send + Sync + Any {
    fn remote_addr(&self) -> SocketAddr;
    fn remote_ip(&self) -> IpAddr;
    fn remote_port(&self) -> u16;
    fn root_ca_path(&self) -> String;
    fn identity(&self) -> Option<&Identity>;
    fn tickrate(&self) -> u64;
    fn unsecure(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any;
}

#[async_trait]
pub trait AppTrait<C>: Send + Sync + Sized
where
    C: AppConfig + Sized,
{
    async fn init(&mut self) -> Result<()>;
    async fn dispatch(&self, io_event: IoEvent) -> Result<(), anyhow::Error>;

    fn ask_secrets(master: &str) -> Result<()>;
    fn ask_secret(master: &str, s: &mut String, stype: SecretType);

    fn is_connected(&self) -> bool;
    fn set_connected(&mut self, connected: bool);

    fn vault(&self) -> Option<&SecretsVault>;
    fn config(&self) -> Box<dyn AppConfig>;

    fn handle_network_response(&mut self, event: IoEvent, res: serde_json::Value);
    fn handle_network_error(&mut self, error: Error);

    async fn run(app: Arc<Mutex<Self>>, notifier: Option<Arc<Notify>>) -> Result<(), anyhow::Error>;
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
        let certificate = get_mgw_root_cert(config)?;
        let builder = reqwest::Client::builder()
            .tls_built_in_root_certs(true)
            .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
            .connect_timeout(Duration::from_secs(15))
            .add_root_certificate(certificate)
            .https_only(true)
            .danger_accept_invalid_certs(config.unsecure());
        let client = if let Some(identity) = config.identity() {
            builder.identity(identity.to_owned()).build()?
        } else {
            builder.build()?
        };

        Ok(Network { app, client, config })
    }

    pub async fn ping_mgw(&mut self) -> Result<(), anyhow::Error> {
        let route = format!("https://{}:{}/swift/mgw/{}", self.config.remote_ip(), self.config.remote_port(), "mgw-monitoring-api/1.0.0/health");
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
                self.app.lock().await.handle_network_error(e.into());
            }
        }
        Ok(())
    }

    async fn handle_io_event(&mut self, io_event: &IoEvent) -> Result<(), anyhow::Error> {
        debug!("Network handling {io_event:?}");
        match io_event {
            IoEvent::Ping => self.ping_mgw().await?,
            IoEvent::GetAllSags | IoEvent::PostSag(_) | IoEvent::DeleteSag(_) => {
                SagHandler::handle(&self.client, self.app, self.config, io_event).await?;
            }
            IoEvent::GetAllCertificates | IoEvent::PostCertificate(_) | IoEvent::DeleteCertificate(_) => {
                CertHandler::handle(&self.client, self.app, self.config, io_event).await?;
            }
            IoEvent::GetAllProfiles | IoEvent::PostProfile(_) | IoEvent::DeleteProfile(_) => {
                ProfileHandler::handle(&self.client, self.app, self.config, io_event).await?;
            }
            IoEvent::GetAllForwardProxyEntity | IoEvent::PostForwardProxyEntity(_) | IoEvent::DeleteForwardProxyEntity(_) => {
                ForwardProxyHandler::handle(&self.client, self.app, self.config, io_event).await?;
            }
            IoEvent::GetAllBusinessApplications | IoEvent::PostBusinessApplication(_) | IoEvent::DeleteBusinessApplication(_) => {
                BusinessApplicationHandler::handle(&self.client, self.app, self.config, io_event).await?;
            }
            IoEvent::GetAllApiClientCredentials | IoEvent::PostApiClientCredential(_) => {
                ApiClientCredentialHandler::handle(&self.client, self.app, self.config, io_event).await?;
            }
            _ => {}
        };
        Ok(())
    }

    #[cfg(feature = "ui")]
    pub async fn handle_network_event(&mut self, io_event: IoEvent) -> Result<(), anyhow::Error> {
        match self.handle_io_event(&io_event).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let mut app = self.app.lock().await;
                app.set_connected(false);
                app.handle_network_error(e);
                Err(Error::msg("Network Error"))
            }
        }
    }

    #[cfg(not(feature = "ui"))]
    pub async fn handle_network_event(&mut self, io_event: IoEvent) -> Result<(), anyhow::Error> {
        match io_event {
            IoEvent::Ping => self.ping_mgw().await?,
            _ => {} // IoEvent::GetAllSags => self.app.lock().await.configuration_state.sags = model::sag::SagEntities::get_all(self.app.lock().await, &self.client, self.config).await?, //sag::get_all_sags(self.app.lock().await, &self.client, self.config).await?,
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
    File::open(config.root_ca_path())?.read_to_end(&mut buf)?;
    Ok(reqwest::Certificate::from_pem(&buf).unwrap())
}

use mgwconf_vault::SecretType;
use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;
use tokio::sync::Mutex;

use crate::{
    api::{
        self,
        configuration::configuration::{ApiKey, Configuration},
    },
    event::IoEvent,
    AppConfig, AppTrait,
};

use super::Handler;

pub(crate) struct CertHandler {}

#[async_trait]
impl<A, C> Handler<A, C> for CertHandler
where
    A: AppTrait<C>,
    C: AppConfig,
{
    async fn handle(client: &Client, app: &Arc<Mutex<A>>, e: &IoEvent) -> Result<(), anyhow::Error> {
        let mut app = app.lock().await;
        match e {
            IoEvent::GetAllCertificates => {
                let entities = api::configuration::certificate_api::certificate_get(
                    &Configuration {
                        base_path: String::from("https://localhost:9003/swift/mgw/mgw-configuration-api/2.0.0"),
                        client: client.clone(),
                        api_key: Some(ApiKey {
                            key: app.vault().as_ref().unwrap().get_secret(SecretType::Configuration).to_owned(),
                            prefix: None,
                        }),
                        ..Default::default()
                    },
                    None,
                )
                .await?;
                app.handle_network_response(IoEvent::GetAllCertificates, entities);
            }
            IoEvent::PostCertificate(entity) => {
                log::debug!("handling {:#?}", entity);
                api::configuration::certificate_api::certificate_create(
                    &Configuration {
                        base_path: String::from("https://localhost:9003/swift/mgw/mgw-configuration-api/2.0.0"),
                        client: client.clone(),
                        api_key: Some(ApiKey {
                            key: app.vault().as_ref().unwrap().get_secret(SecretType::Configuration).to_owned(),
                            prefix: None,
                        }),
                        ..Default::default()
                    },
                    entity.clone(),
                )
                .await?;
                app.handle_network_response(e.clone(), "".into());
            }
            IoEvent::DeleteCertificate(e) => {
                api::configuration::certificate_api::certificate_delete(
                    &Configuration {
                        base_path: String::from("https://localhost:9003/swift/mgw/mgw-configuration-api/2.0.0"),
                        client: client.clone(),
                        api_key: Some(ApiKey {
                            key: app.vault().as_ref().unwrap().get_secret(SecretType::Configuration).to_owned(),
                            prefix: None,
                        }),
                        ..Default::default()
                    },
                    &e.alias,
                )
                .await?;
            }
            _ => {}
        }
        Ok(())
    }
}

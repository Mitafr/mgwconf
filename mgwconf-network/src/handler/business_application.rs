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

pub(crate) struct BusinessApplicationHandler {}

#[async_trait]
impl<A, C> Handler<A, C> for BusinessApplicationHandler
where
    A: AppTrait<C>,
    C: AppConfig,
{
    async fn handle(client: &Client, app: &Arc<Mutex<A>>, e: &IoEvent) -> Result<(), anyhow::Error> {
        let mut app = app.lock().await;
        match e {
            IoEvent::GetAllBusinessApplications => {
                let entities = api::configuration::business_application_api::business_application_get(
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
                app.handle_network_response(IoEvent::GetAllBusinessApplications, entities);
            }
            IoEvent::PostBusinessApplication(entity) => {
                log::debug!("handling {:#?}", entity);
                api::configuration::business_application_api::business_application_create(
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
            IoEvent::DeleteBusinessApplication(e) => {
                api::configuration::business_application_api::business_application_delete(
                    &Configuration {
                        base_path: String::from("https://localhost:9003/swift/mgw/mgw-configuration-api/2.0.0"),
                        client: client.clone(),
                        api_key: Some(ApiKey {
                            key: app.vault().as_ref().unwrap().get_secret(SecretType::Configuration).to_owned(),
                            prefix: None,
                        }),
                        ..Default::default()
                    },
                    &e.application_name,
                )
                .await?;
            }
            _ => {}
        }
        Ok(())
    }
}

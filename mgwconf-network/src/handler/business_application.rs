use mgwconf_vault::SecretType;
use std::sync::Arc;

use async_trait::async_trait;
use mgw_configuration::apis::{
    business_application_api,
    configuration::{ApiKey, Configuration},
};
use reqwest::Client;
use tokio::sync::Mutex;

use crate::{event::IoEvent, AppConfig, AppTrait};

use super::{base_url, Handler};

pub(crate) struct BusinessApplicationHandler {}

#[async_trait]
impl<A, C> Handler<A, C> for BusinessApplicationHandler
where
    A: AppTrait<C>,
    C: AppConfig,
{
    async fn handle(
        client: &Client,
        app: &Arc<Mutex<A>>,
        config: &C,
        e: &IoEvent,
    ) -> Result<(), anyhow::Error> {
        let mut app = app.lock().await;
        match e {
            IoEvent::GetAllBusinessApplications => {
                let entities = business_application_api::business_application_get(
                    &Configuration {
                        base_path: format!(
                            "{}/swift/mgw/mgw-configuration-api/2.0.0",
                            base_url(config)
                        ),
                        client: client.clone(),
                        api_key: Some(ApiKey {
                            key: app
                                .vault()
                                .as_ref()
                                .unwrap()
                                .get_secret(SecretType::Configuration)
                                .to_owned(),
                            prefix: None,
                        }),
                        ..Default::default()
                    },
                    None,
                )
                .await?;
                app.handle_network_response(
                    IoEvent::GetAllBusinessApplications,
                    serde_json::to_value(entities.entity.unwrap()).unwrap(),
                );
            }
            IoEvent::PostBusinessApplication(entity) => {
                log::debug!("handling {:#?}", entity);
                business_application_api::business_application_create(
                    &Configuration {
                        base_path: format!(
                            "{}/swift/mgw/mgw-configuration-api/2.0.0",
                            base_url(config)
                        ),
                        client: client.clone(),
                        api_key: Some(ApiKey {
                            key: app
                                .vault()
                                .as_ref()
                                .unwrap()
                                .get_secret(SecretType::Configuration)
                                .to_owned(),
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
                business_application_api::business_application_delete(
                    &Configuration {
                        base_path: format!(
                            "{}/swift/mgw/mgw-configuration-api/2.0.0",
                            base_url(config)
                        ),
                        client: client.clone(),
                        api_key: Some(ApiKey {
                            key: app
                                .vault()
                                .as_ref()
                                .unwrap()
                                .get_secret(SecretType::Configuration)
                                .to_owned(),
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

use mgw_configuration::apis::{
    api_client_credentials_api::{api_credentials_info_create, api_credentials_info_get},
    configuration::{ApiKey, Configuration},
};
use mgwconf_vault::SecretType;
use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;
use tokio::sync::Mutex;

use crate::{event::IoEvent, AppConfig, AppTrait};

use super::{base_url, Handler};

pub(crate) struct ApiClientCredentialHandler {}

#[async_trait]
impl<A, C> Handler<A, C> for ApiClientCredentialHandler
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
            IoEvent::GetAllApiClientCredentials => {
                let entities = api_credentials_info_get(
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
                app.handle_network_response(IoEvent::GetAllApiClientCredentials, entities);
            }
            IoEvent::PostApiClientCredential(entity) => {
                log::debug!("handling {:#?}", entity);
                let r = api_credentials_info_create(
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
                    entity.to_owned(),
                )
                .await?;
                app.handle_network_response(e.clone(), r);
            }
            _ => {}
        }
        Ok(())
    }
}

use mgw_configuration::apis::{
    configuration::{ApiKey, Configuration},
    forward_proxy_api,
};
use mgwconf_vault::SecretType;
use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;
use tokio::sync::Mutex;

use crate::{event::IoEvent, AppConfig, AppTrait};

use super::{base_url, Handler};

pub(crate) struct ForwardProxyHandler {}

#[async_trait]
impl<A, C> Handler<A, C> for ForwardProxyHandler
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
            IoEvent::GetAllForwardProxyEntity => {
                let entities = forward_proxy_api::forward_proxies_info_get(
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
                    None,
                )
                .await?;
                app.handle_network_response(IoEvent::GetAllForwardProxyEntity, entities);
            }
            IoEvent::PostForwardProxyEntity(entity) => {
                log::debug!("handling {:#?}", entity);
                let res = forward_proxy_api::forward_proxy_info_create(
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
                app.handle_network_response(e.clone(), res);
            }
            IoEvent::DeleteForwardProxyEntity(e) => {
                forward_proxy_api::forward_proxy_info_delete(
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
                    &e.hostname,
                    &e.port.to_string(),
                )
                .await?;
            }
            _ => {}
        }
        Ok(())
    }
}

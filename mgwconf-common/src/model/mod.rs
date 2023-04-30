use std::{fmt::Debug, any::Any};

use async_trait::async_trait;
use crate::{AppTrait, config::Config};
use reqwest::Client;
use tokio::sync::MutexGuard;

///
/// Contains the entity itself
///
pub trait InnerEntityTrait: Debug + Send + Sync {
    fn get_name(&self) -> &str;
    fn to_string(&self) -> String {
        String::new()
    }
}

///
/// Contains a collection of entities
///
pub trait CollectionEntityTrait: Debug {
    fn as_any(&self) -> &dyn Any;
}

#[async_trait]
pub trait ModelTrait<A>
where A: AppTrait
{
    type Collection: CollectionEntityTrait;

    type Entity: InnerEntityTrait;

    async fn get(app: MutexGuard<'_, A>, client: &Client, config: &Config) -> Result<Self::Entity, anyhow::Error>;
    async fn get_all(app: MutexGuard<'_, A>, client: &Client, config: &Config) -> Result<Self::Collection, anyhow::Error>;
    async fn post(app: MutexGuard<'_, A>, client: &Client, config: &Config) -> Result<(), anyhow::Error>;
    async fn delete(app: MutexGuard<'_, A>, client: &Client, config: &Config) -> Result<(), anyhow::Error>;
}

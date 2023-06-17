use std::any::Any;
use std::fmt::Debug;

use async_trait::async_trait;
use reqwest::Client;
use tokio::sync::MutexGuard;

use crate::{AppConfig, AppTrait};

pub mod business_application;
pub mod certificate;
pub mod prelude;
pub mod profile;
pub mod sag;

///
/// Contains the entity itself
///
pub trait InnerEntityTrait: Debug + Send + Sync {
    fn name(&self) -> &str;
    fn delete_query(&self) -> Vec<(String, String)>;
    fn to_string(&self) -> String {
        String::new()
    }
    fn as_any(&self) -> &dyn Any;
}

///
/// Contains a collection of entities
///
pub trait CollectionEntityTrait: Debug {
    fn as_any(&self) -> &dyn Any;

    fn get(&self, index: usize) -> Option<Box<dyn InnerEntityTrait>>;
}

#[async_trait]
pub trait ModelTrait<A, C>
where
    A: AppTrait<C>,
    C: AppConfig,
{
    type Collection: CollectionEntityTrait;

    type Entity: InnerEntityTrait;

    async fn get(app: &MutexGuard<'_, A>, client: &Client, config: &C) -> Result<Self::Entity, anyhow::Error>;
    async fn get_all(app: &MutexGuard<'_, A>, client: &Client, config: &C) -> Result<Self::Collection, anyhow::Error>;
    async fn post(app: &MutexGuard<'_, A>, client: &Client, config: &C) -> Result<(), anyhow::Error>;
    async fn delete(app: &MutexGuard<'_, A>, client: &Client, config: &C, e: &Self::Entity) -> Result<(), anyhow::Error>;
}

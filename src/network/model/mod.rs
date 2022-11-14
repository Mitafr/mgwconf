use async_trait::async_trait;
use reqwest::Client;
use tokio::sync::MutexGuard;

use crate::{app::App, config::Config};

pub mod business_application;
pub mod certificate;
mod prelude;
pub mod sag;

pub trait InnerEntityTrait {}

pub trait EntityTrait {}

#[async_trait]
pub trait ModelTrait {
    type Entity: EntityTrait;

    type Inner: InnerEntityTrait;

    async fn get(app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<Self::Entity, anyhow::Error>;
    async fn post(app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<(), anyhow::Error>;
}

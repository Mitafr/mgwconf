use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;
use tokio::sync::Mutex;

pub(super) mod cert;
pub(super) mod sag;

use crate::{event::IoEvent, AppConfig, AppTrait};

#[async_trait]
pub trait Handler<A, C>
where
    A: AppTrait<C>,
    C: AppConfig,
{
    async fn handle(client: &Client, app: &Arc<Mutex<A>>, e: &IoEvent)
        -> Result<(), anyhow::Error>;
}

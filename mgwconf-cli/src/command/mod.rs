use async_trait::async_trait;

use crate::app::CliApp;

pub mod get_all;
pub mod get_api_client_credential;
pub mod get_business_application;
pub mod get_certificate;
pub mod get_profile;
pub mod get_proxy;
pub mod get_sag;
pub mod registry;

pub trait CommandRegistryTrait {
    fn execute(&self, app: CliApp) -> std::pin::Pin<Box<dyn std::future::Future<Output = usize> + Send>>;

    fn name(&self) -> &'static str;
}

#[async_trait]
pub trait CommandTrait {
    async fn execute(app: &CliApp);

    fn num_op() -> usize;
}

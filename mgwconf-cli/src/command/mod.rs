use crate::app::CliApp;

pub mod get_all;
pub mod get_certificate;
pub mod get_sag;
pub mod registry;

pub trait CommandTrait {
    fn execute(&self, app: CliApp) -> std::pin::Pin<Box<dyn std::future::Future<Output = usize> + Send>>;

    fn name(&self) -> &'static str;
}

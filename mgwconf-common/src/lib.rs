use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;

use config::Config;
pub use mgwconf_vault::*;
use model::CollectionEntityTrait;

pub mod config;
pub mod model;

#[async_trait]
pub trait AppTrait: Send {
    async fn init(&mut self) -> Result<()>;
    fn dispatch(&mut self, io_event: IoEvent) -> Result<()>;

    fn ask_secrets(&mut self) -> Result<()>;
    fn ask_secret(&mut self, s: &mut String, stype: SecretType);

    fn is_connected(&self) -> bool;
    fn set_connected(&mut self, connected: bool);

    fn get_vault(&self) -> Option<&SecretsVault>;
    fn get_config(&self) -> Option<&Config>;

    fn handle_network_response<T>(&mut self, event: IoEvent, res: T)
    where
        T: CollectionEntityTrait;
}

#[derive(Debug)]
pub enum IoEvent {
    Ping,
    GetAllBusinessApplications,
    GetAllCertificates,
    GetAllSags,
    PostBusinessApplication,
    PostCertificate,
    PostSag,
    DeleteBusinessApplication,
    DeleteCertificate,
    DeleteSag,
}

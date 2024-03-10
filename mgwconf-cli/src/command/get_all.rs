use mgwconf_network::AppTrait;

use crate::{app::CliApp, config::Config};

pub struct GetAll {}

impl GetAll {
    pub async fn execute(app: &CliApp) {
        <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllForwardProxyEntity).await.unwrap();
        <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllBusinessApplications).await.unwrap();
        //<CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllApplicationProfileEntity).await.unwrap();
        <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllCertificates).await.unwrap();
        <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllSags).await.unwrap();
        // <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllProfiles),
    }
}

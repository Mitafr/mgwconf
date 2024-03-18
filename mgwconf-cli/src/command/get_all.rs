use async_trait::async_trait;
use mgwconf_network::AppTrait;

use crate::{app::CliApp, config::Config};

use super::CommandTrait;

pub struct GetAll {}

#[async_trait]
impl CommandTrait for GetAll {
    async fn execute(app: &CliApp) {
        tokio::try_join!(
            <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllForwardProxyEntity),
            <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllBusinessApplications),
            <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllCertificates),
            <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllSags),
            <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllProfiles),
            // <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllApplicationProfileEntity),
        )
        .unwrap();
    }

    fn num_op() -> usize {
        5
    }
}

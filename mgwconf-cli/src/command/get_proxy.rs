use async_trait::async_trait;
use mgwconf_network::AppTrait;
use std::fs::{File, OpenOptions};

use crate::{app::CliApp, config::Config};

use super::CommandTrait;

pub struct GetProxy {}

impl GetProxy {
    pub fn output_file() -> File {
        OpenOptions::new().append(true).create(true).truncate(false).open("output/proxies").unwrap()
    }
}

#[async_trait]
impl CommandTrait for GetProxy {
    async fn execute(app: &CliApp) {
        <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllForwardProxyEntity).await.unwrap();
    }

    fn num_op() -> usize {
        1
    }
}

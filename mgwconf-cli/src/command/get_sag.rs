use async_trait::async_trait;
use mgwconf_network::AppTrait;
use std::fs::{File, OpenOptions};

use crate::{app::CliApp, config::Config};

use super::CommandTrait;

pub struct GetSag {}

impl GetSag {
    pub fn output_file() -> File {
        OpenOptions::new().append(true).create(true).truncate(false).open("output/sags").unwrap()
    }
}

#[async_trait]
impl CommandTrait for GetSag {
    async fn execute(app: &CliApp) {
        <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllSags).await.unwrap();
    }

    fn num_op() -> usize {
        1
    }
}

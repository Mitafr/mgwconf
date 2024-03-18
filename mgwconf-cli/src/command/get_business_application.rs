use async_trait::async_trait;
use mgwconf_network::AppTrait;
use std::fs::{File, OpenOptions};

use crate::{app::CliApp, config::Config};

use super::CommandTrait;

pub struct GetBusinessApplication {}

impl GetBusinessApplication {
    pub fn output_file() -> File {
        OpenOptions::new().append(true).create(true).truncate(false).open("output/business_applications").unwrap()
    }
}

#[async_trait]
impl CommandTrait for GetBusinessApplication {
    async fn execute(app: &CliApp) {
        <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllBusinessApplications).await.unwrap();
    }

    fn num_op() -> usize {
        1
    }
}

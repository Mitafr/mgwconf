use mgwconf_network::AppTrait;
use std::fs::{File, OpenOptions};

use crate::{app::CliApp, config::Config};

pub struct GetSag {}

impl GetSag {
    pub async fn execute(app: &CliApp) {
        <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllSags).await.unwrap();
    }

    pub fn output_file() -> File {
        OpenOptions::new().append(true).create(true).truncate(false).open("output/get_sags").unwrap()
    }
}

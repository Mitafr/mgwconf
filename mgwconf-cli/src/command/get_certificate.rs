use mgwconf_network::AppTrait;
use std::fs::File;

use crate::{app::CliApp, config::Config};

pub struct GetCertificate {}

impl GetCertificate {
    pub async fn execute(app: &CliApp) {
        <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllCertificates).await.unwrap();
    }

    pub fn output_file() -> File {
        File::create("output/get_certificates").unwrap()
    }
}

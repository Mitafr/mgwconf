use async_trait::async_trait;
use mgwconf_network::AppTrait;
use std::fs::File;

use crate::{app::CliApp, config::Config};

use super::CommandTrait;

pub struct GetCertificate {}

impl GetCertificate {
    pub fn output_file() -> File {
        File::create("output/get_certificates").unwrap()
    }
}

#[async_trait]
impl CommandTrait for GetCertificate {
    async fn execute(app: &CliApp) {
        <CliApp as AppTrait<Config>>::dispatch(app, mgwconf_network::event::IoEvent::GetAllCertificates).await.unwrap();
    }

    fn num_op() -> usize {
        1
    }
}

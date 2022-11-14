use log::debug;
use rand::distributions::{Alphanumeric, DistString};
use reqwest::{Client, StatusCode};
use tokio::sync::MutexGuard;

use crate::{
    config::Config,
    network::{
        model::business_application::{BusinessApplication, BusinessApplications},
        utils::route_url,
    },
};
use crate::{network::utils::generate_api_header, ui::prelude::App};

pub async fn get_all_business_applications(mut app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<(), anyhow::Error> {
    let header = generate_api_header(&app, crate::app::vault::SecretType::Configuration);
    let res = client.get(route_url(config, "mgw-configuration-api/2.0.0/business-application")).headers(header).send().await?;
    if ![StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
        return Err(anyhow::Error::msg(format!("{:?}", res)));
    }
    debug!("{:#?}", res);
    app.configuration_state.business_applications = res.json::<BusinessApplications>().await?;
    Ok(())
}

pub async fn post_business_application(mut app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<(), anyhow::Error> {
    let header = generate_api_header(&app, crate::app::vault::SecretType::Configuration);
    let rngsecret = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    let test = BusinessApplication {
        application_name: "test2".to_owned(),
        shared_secret: rngsecret,
    };
    client.post(route_url(config, "mgw-configuration-api/2.0.0/business-application")).json(&test).headers(header).send().await?;
    app.configuration_state.business_applications.0.push(test);
    Ok(())
}

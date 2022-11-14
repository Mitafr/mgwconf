use log::debug;
use reqwest::{Client, StatusCode};
use tokio::sync::MutexGuard;

use crate::config::Config;
use crate::network::model::*;
use crate::network::utils::route_url;
use crate::{network::utils::generate_api_header, ui::prelude::App};

pub async fn get_all_sags(mut app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<(), anyhow::Error> {
    let header = generate_api_header(&app, crate::app::vault::SecretType::Configuration);
    let res = client.get(route_url(config, "mgw-configuration-api/2.0.0/sag")).headers(header).send().await?;
    if ![StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
        return Err(anyhow::Error::msg(format!("{:?}", res)));
    }
    debug!("{:#?}", res);
    app.configuration_state.sags = res.json::<sag::SagEntities>().await?;
    Ok(())
}

pub async fn post_sag(mut app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<(), anyhow::Error> {
    let header = generate_api_header(&app, crate::app::vault::SecretType::Configuration);
    let test = sag::SagEntity {
        hostname: "test2".to_owned(),
        port: 48002,
        message_partner_name: Some("Sag MP".to_owned()),
        lau_key: Some("Abcd1234Abcd1234Abcd1234Abcd1234".to_owned()),
        ssl_dn: Some("ssl".to_owned()),
        user_dns: vec!["cn=apitest,ou=apicore,o=swhqbebb,o=swift".to_owned()],
        active: true,
        public_certificate_alias: Some("test".to_owned()),
    };
    client.post(route_url(config, "mgw-configuration-api/2.0.0/sag")).json(&test).headers(header).send().await?;
    app.configuration_state.sags.0.push(test);
    Ok(())
}

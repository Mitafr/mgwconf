use log::debug;
use reqwest::{Client, StatusCode};
use tokio::sync::MutexGuard;

use crate::{
    config::Config,
    network::{
        model::certificate::{CertificateEntities, CertificateEntity},
        utils::route_url,
    },
};
use crate::{network::utils::generate_api_header, ui::prelude::App};

pub async fn get_all_certificates(mut app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<(), anyhow::Error> {
    let header = generate_api_header(&app, crate::app::vault::SecretType::Configuration);

    let res = client.get(route_url(config, "mgw-configuration-api/2.0.0/certificate")).headers(header).send().await?;
    if ![StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
        return Err(anyhow::Error::msg(format!("{:?}", res)));
    }
    debug!("{:#?}", res);
    app.configuration_state.certificates = res.json::<CertificateEntities>().await?;
    Ok(())
}

pub async fn post_certificate(mut app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<(), anyhow::Error> {
    let header = generate_api_header(&app, crate::app::vault::SecretType::Configuration);
    let test = CertificateEntity {
        alias: "test".to_owned(),
        certificate_x509: std::fs::read_to_string("./mgw.cer").unwrap(),
        ..Default::default()
    };
    client.post(route_url(config, "mgw-configuration-api/2.0.0/certificate")).json(&test).headers(header).send().await?;
    app.configuration_state.certificates.0.push(test);
    Ok(())
}

use std::{fs::File, io::Read, sync::Arc};

use anyhow::Result;
use log::debug;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Certificate, Client, StatusCode,
};
use tokio::sync::Mutex;

use crate::{app::App, config::Config, network::model::sag::SagEntities};

use self::model::{
    certificate::{CertificateEntities, CertificateEntity},
    sag::SagEntity,
};

pub mod model;

#[derive(Debug)]
pub enum IoEvent {
    Ping,
    GetAllSags,
    GetAllCertificate,
    PostSag,
    PostCertificate,
}

#[derive(Clone)]
pub struct Network<'a> {
    pub app: &'a Arc<Mutex<App>>,
    client: Client,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>, config: &Config) -> Result<Self> {
        let certificate = get_mgw_root_cert(config)?;
        let client = reqwest::Client::builder().local_address(config.remote_ip).add_root_certificate(certificate).build()?;
        Ok(Network { app, client })
    }

    pub async fn ping_mgw(&mut self) -> Result<(), anyhow::Error> {
        let res = self.client.get("https://localhost:9003/swift/mgw/mgw-monitoring-api/1.0.0/health").send().await?;
        let mut app = self.app.lock().await;
        if [StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
            app.connectivity_test = false;
        } else {
            app.connectivity_test = true;
        }
        Ok(())
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) -> Result<(), anyhow::Error> {
        match io_event {
            IoEvent::Ping => self.ping_mgw().await?,
            IoEvent::GetAllSags => self.get_all_sags().await?,
            IoEvent::PostSag => self.post_sag().await?,
            IoEvent::GetAllCertificate => self.get_all_certificates().await?,
            IoEvent::PostCertificate => self.post_certificate().await?,
        };
        Ok(())
    }

    pub async fn get_all_sags(&mut self) -> Result<(), anyhow::Error> {
        let mut app = self.app.lock().await;
        let mut header = HeaderMap::new();
        header.append(
            "X-API-KEY",
            HeaderValue::from_str(app.vault.as_ref().expect("Vault not initialized correctly").configuration.as_ref().unwrap()).unwrap(),
        );
        let res = self.client.get("https://localhost:9003/swift/mgw/mgw-configuration-api/2.0.0/sag").headers(header).send().await?;
        if ![StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
            return Err(anyhow::Error::msg(format!("{:?}", res)));
        }
        debug!("{:#?}", res);
        app.configuration_state.sags = res.json::<SagEntities>().await?;
        Ok(())
    }

    pub async fn post_sag(&mut self) -> Result<(), anyhow::Error> {
        let mut app = self.app.lock().await;
        let mut header = HeaderMap::new();
        header.append(
            "X-API-KEY",
            HeaderValue::from_str(app.vault.as_ref().expect("Vault not initialized correctly").configuration.as_ref().unwrap()).unwrap(),
        );
        let test = SagEntity {
            hostname: "test2".to_owned(),
            port: 48002,
            message_partner_name: Some("Sag MP".to_owned()),
            lau_key: Some("Abcd1234Abcd1234Abcd1234Abcd1234".to_owned()),
            ssl_dn: Some("ssl".to_owned()),
            user_dns: vec!["cn=apitest,ou=apicore,o=swhqbebb,o=swift".to_owned()],
            active: true,
            public_certificate_alias: Some("test".to_owned()),
        };
        self.client.post("https://localhost:9003/swift/mgw/mgw-configuration-api/2.0.0/sag").json(&test).headers(header).send().await?;
        app.configuration_state.sags.0.push(test);
        Ok(())
    }

    pub async fn get_all_certificates(&mut self) -> Result<(), anyhow::Error> {
        let mut app = self.app.lock().await;
        let mut header = HeaderMap::new();
        header.append(
            "X-API-KEY",
            HeaderValue::from_str(app.vault.as_ref().expect("Vault not initialized correctly").configuration.as_ref().unwrap()).unwrap(),
        );
        let res = self.client.get("https://localhost:9003/swift/mgw/mgw-configuration-api/2.0.0/certificate").headers(header).send().await?;
        if ![StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
            return Err(anyhow::Error::msg(format!("{:?}", res)));
        }
        debug!("{:#?}", res);
        app.configuration_state.certificates = res.json::<CertificateEntities>().await?;
        Ok(())
    }

    pub async fn post_certificate(&mut self) -> Result<(), anyhow::Error> {
        let mut app = self.app.lock().await;
        let mut header = HeaderMap::new();
        header.append(
            "X-API-KEY",
            HeaderValue::from_str(app.vault.as_ref().expect("Vault not initialized correctly").configuration.as_ref().unwrap()).unwrap(),
        );
        let test = CertificateEntity {
            alias: "test".to_owned(),
            certificate_x509: std::fs::read_to_string("./mgw.cer").unwrap(),
            ..Default::default()
        };
        self.client
            .post("https://localhost:9003/swift/mgw/mgw-configuration-api/2.0.0/certificate")
            .json(&test)
            .headers(header)
            .send()
            .await?;
        app.configuration_state.certificates.0.push(test);
        Ok(())
    }
}

fn get_mgw_root_cert(config: &Config) -> anyhow::Result<Certificate> {
    let mut buf = Vec::new();
    File::open(&config.root_ca_path)?.read_to_end(&mut buf)?;
    Ok(reqwest::Certificate::from_pem(&buf).unwrap())
}

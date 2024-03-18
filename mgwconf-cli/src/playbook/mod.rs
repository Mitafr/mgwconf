use std::fs::{read_to_string, File};
use std::path::PathBuf;

use mgwconf_network::model::configuration::{ApplicationProfileEntity, BusinessApplicationEntity, CertificateEntity, ForwardProxyEntity};
use mgwconf_network::{event::IoEvent, model::configuration::SagEntity, AppTrait};
use serde::{Deserialize, Serialize};

use crate::app::CliApp;
use crate::config::Config;

use self::error::PlaybookError;

pub mod error;

#[derive(Debug, Clone)]
pub struct Playbook {
    pub path: PathBuf,
}

impl Playbook {
    pub async fn process(&self, app: &CliApp) -> Result<usize, PlaybookError> {
        log::info!("Loading playbook at {:?}", self.path);
        let file = File::open(&self.path)?;
        let entries = serde_yaml::from_reader::<File, PlaybookEntries>(file)?;
        let mut num_op = 0;
        for i in entries.commands {
            num_op += match i {
                CommandType::Delete(e) => self.process_delete(e, app).await?,
                CommandType::Create(e) => self.process_create(e, app).await?,
            }
        }
        Ok(num_op)
    }

    async fn process_delete(&self, _e: EntityType, _app: &CliApp) -> Result<usize, PlaybookError> {
        todo!();
    }

    async fn process_create(&self, e: EntityType, app: &CliApp) -> Result<usize, PlaybookError> {
        let mut num_op = 0;
        macro_rules! handle {
            ($h:tt, $io:expr, $e:tt) => {
                if $h.json.is_none() && $h.file.is_none() {
                    return Err(PlaybookError::MalformedPlaybook("Sag import must contains either file or json input"));
                }
                if let Some(j) = $h.json {
                    <CliApp as AppTrait<Config>>::dispatch(app, $io(serde_json::from_str::<$e>(&j)?)).await?;
                } else {
                    let file = read_to_string::<String>($h.file.unwrap().into())?;
                    let entities: Vec<$e> = serde_json::from_str(&file)?;
                    for e in entities {
                        <CliApp as AppTrait<Config>>::dispatch(app, $io(e)).await?;
                    }
                }
                num_op += 1;
            };
        }
        match e {
            EntityType::Sag(h) => {
                handle!(h, IoEvent::PostSag, SagEntity);
            }
            EntityType::Proxy(p) => {
                handle!(p, IoEvent::PostForwardProxyEntity, ForwardProxyEntity);
            }
            EntityType::Profile(p) => {
                handle!(p, IoEvent::PostProfile, ApplicationProfileEntity);
            }
            EntityType::BusinessApplication(b) => {
                handle!(b, IoEvent::PostBusinessApplication, BusinessApplicationEntity);
            }
            EntityType::Certificate(c) => {
                handle!(c, IoEvent::PostCertificate, CertificateEntity);
            }
        }
        Ok(num_op)
    }
}

impl From<String> for Playbook {
    fn from(value: String) -> Self {
        Playbook { path: value.into() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PlaybookEntries {
    commands: Vec<CommandType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum CommandType {
    Delete(EntityType),
    Create(EntityType),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "entity_type")]
enum EntityType {
    Sag(SagImport),
    Proxy(ProxyImport),
    Profile(ProfileImport),
    BusinessApplication(BusinessApplicationImport),
    Certificate(CertificateImport),
}

#[derive(Debug, Serialize, Deserialize)]
struct SagImport {
    file: Option<String>,
    json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProxyImport {
    file: Option<String>,
    json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProfileImport {
    file: Option<String>,
    json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BusinessApplicationImport {
    file: Option<String>,
    json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CertificateImport {
    file: Option<String>,
    json: Option<String>,
}

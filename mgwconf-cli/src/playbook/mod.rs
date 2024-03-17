use std::path::PathBuf;
use std::{fs::File, str::FromStr};

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
    pub async fn process(&self, app: &CliApp) -> Result<(), PlaybookError> {
        log::info!("Loading playbook at {:?}", self.path);
        let file = File::open(&self.path)?;
        let import = serde_yaml::from_reader::<File, Import>(file)?;
        for i in import.import {
            match i {
                ImportType::Sag(s) => {
                    if s.json.is_none() {
                        return Err(PlaybookError::MalformedPlaybook("Sag import contains empty json"));
                    }
                    <CliApp as AppTrait<Config>>::dispatch(app, IoEvent::PostSag(SagEntity::from_str(&s.json.unwrap())?)).await?;
                }
                ImportType::Proxy(_p) => todo!(),
            }
        }
        Ok(())
    }
}

impl From<String> for Playbook {
    fn from(value: String) -> Self {
        Playbook { path: value.into() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Import {
    import: Vec<ImportType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ImportType {
    Sag(SagImport),
    Proxy(ProxyImport),
}

#[derive(Debug, Serialize, Deserialize)]
struct SagImport {
    json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProxyImport {
    json: Option<String>,
}

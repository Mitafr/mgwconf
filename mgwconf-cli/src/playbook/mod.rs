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
        let entries = serde_yaml::from_reader::<File, PlaybookEntries>(file)?;
        for i in entries.commands {
            match i {
                CommandType::Delete(e) => self.process_delete(e, app).await?,
                CommandType::Create(e) => self.process_create(e, app).await?,
            }
        }
        Ok(())
    }

    async fn process_delete(&self, _e: EntityType, _app: &CliApp) -> Result<(), PlaybookError> {
        todo!();
    }

    async fn process_create(&self, e: EntityType, app: &CliApp) -> Result<(), PlaybookError> {
        match e {
            EntityType::Sag(s) => {
                if s.json.is_none() {
                    return Err(PlaybookError::MalformedPlaybook("Sag import contains empty json"));
                }
                <CliApp as AppTrait<Config>>::dispatch(app, IoEvent::PostSag(SagEntity::from_str(&s.json.unwrap())?)).await?;
            }
            EntityType::Proxy(_p) => todo!(),
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
}

#[derive(Debug, Serialize, Deserialize)]
struct SagImport {
    json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProxyImport {
    json: Option<String>,
}

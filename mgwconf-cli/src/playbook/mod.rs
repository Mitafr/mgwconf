use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::{read_to_string, File};
use std::hash::Hash;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use mgwconf_network::mgw_configuration::models::BusinessApplicationEntity;
use mgwconf_network::model::configuration::{
    ApplicationProfileEntity, CertificateEntity, ForwardProxyEntity,
};
use mgwconf_network::{event::IoEvent, model::configuration::SagEntity, AppTrait};
use serde::{Deserialize, Deserializer, Serialize};

use crate::app::CliApp;
use crate::config::Config;

use self::error::PlaybookError;

pub mod error;

#[derive(Debug, Clone)]
pub struct Playbook {
    pub path: PathBuf,
    pub entries: PlaybookEntries,
}

impl Playbook {
    pub fn read<P: AsRef<Path> + Debug>(path: P) -> Result<Playbook, PlaybookError> {
        log::info!("Loading playbook at {:?}", path);
        let file = File::open(&path)?;
        let entries = serde_yaml::from_reader::<File, PlaybookEntries>(file)?;
        Ok(Playbook {
            path: path.as_ref().to_path_buf(),
            entries,
        })
    }

    pub async fn process(&self, app: &CliApp) -> Result<usize, PlaybookError> {
        log::info!("Processing playbook");
        let mut num_op = 0;
        for i in self.entries.commands.iter() {
            num_op += match i {
                CommandType::Delete(e) => self.process_delete(e, app).await?,
                CommandType::Create(e) => self.process_create(e, app).await?,
            }
        }
        Ok(num_op)
    }

    async fn process_delete(&self, _e: &EntityType, _app: &CliApp) -> Result<usize, PlaybookError> {
        todo!();
    }

    async fn process_create(&self, e: &EntityType, app: &CliApp) -> Result<usize, PlaybookError> {
        let mut num_op = 0;
        macro_rules! handle {
            ($h:tt, $io:expr, $e:tt) => {
                if $h.json.is_none() && $h.file.is_none() {
                    return Err(PlaybookError::MalformedPlaybook(
                        "Sag import must contains either file or json input",
                    ));
                }
                if let Some(j) = &$h.json {
                    <CliApp as AppTrait<Config>>::dispatch(
                        app,
                        $io(serde_json::from_str::<$e>(&j)?),
                    )
                    .await?;
                } else {
                    let file = read_to_string::<&str>(&$h.file.as_ref().unwrap())?;
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
                handle!(
                    b,
                    IoEvent::PostBusinessApplication,
                    BusinessApplicationEntity
                );
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
        match Playbook::read(value) {
            Ok(p) => p,
            Err(e) => panic!("Cannot read playbook {e}"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlaybookEntries {
    #[serde(deserialize_with = "deserialize_hosts")]
    pub hosts: Vec<SocketAddr>,
    commands: Vec<CommandType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
enum CommandType {
    Delete(EntityType),
    Create(EntityType),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "entity_type")]
enum EntityType {
    Sag(SagImport),
    Proxy(ProxyImport),
    Profile(ProfileImport),
    BusinessApplication(BusinessApplicationImport),
    Certificate(CertificateImport),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SagImport {
    file: Option<String>,
    json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProxyImport {
    file: Option<String>,
    json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProfileImport {
    file: Option<String>,
    json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BusinessApplicationImport {
    file: Option<String>,
    json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CertificateImport {
    file: Option<String>,
    json: Option<String>,
}

fn deserialize_hosts<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Hash + Eq + Clone + Debug,
{
    let vec: Vec<T> = Vec::deserialize(deserializer)?;
    let mut found = HashSet::new();
    let dup = vec
        .iter()
        .filter(|&element| !found.insert(element.clone()))
        .cloned()
        .collect::<Vec<T>>();
    if !dup.is_empty() {
        Err(serde::de::Error::custom(format!(
            "hosts cannot contains duplicate data {dup:#?}"
        )))
    } else if vec.is_empty() {
        Err(serde::de::Error::custom("Empty hosts not allowed"))
    } else {
        Ok(vec)
    }
}

use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlaybookError {
    #[error("Cannot read playbook {0}")]
    IoErr(#[from] io::Error),
    #[error("Cannot read yaml playbook {0}")]
    DeYamlErr(#[from] serde_yaml::Error),
    #[error("Cannot read json in playbook {0}")]
    DeJsonErr(#[from] serde_json::Error),
    #[error("Cannot dispatch playbook actions {0}")]
    DispatchErr(#[from] anyhow::Error),
    #[error("Playbook contains malformed data {0}")]
    MalformedPlaybook(&'static str),
}

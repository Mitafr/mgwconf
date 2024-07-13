use std::slice::Iter;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[cfg(feature = "store")]
mod error;
#[cfg(not(feature = "store"))]
mod prompt;
#[cfg(feature = "store")]
mod store;

#[derive(Copy, Clone, Debug, Default, Zeroize)]
pub enum SecretType {
    #[default]
    Configuration,
    Monitoring,
    Management,
    Encrypt,
}

impl std::fmt::Display for SecretType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretType::Configuration => write!(f, "Configuration"),
            SecretType::Monitoring => write!(f, "Monitoring"),
            SecretType::Management => write!(f, "Management"),
            SecretType::Encrypt => write!(f, "Encrypt"),
        }
    }
}

impl SecretType {
    pub fn iterator() -> Iter<'static, SecretType> {
        static SECRETTYPES: [SecretType; 4] = [
            SecretType::Configuration,
            SecretType::Monitoring,
            SecretType::Management,
            SecretType::Encrypt,
        ];
        SECRETTYPES.iter()
    }
}

#[cfg(not(feature = "store"))]
#[derive(Default, Debug, Zeroize, ZeroizeOnDrop, Clone)]
pub struct SecretsVault {
    configuration: String,
    monitoring: String,
    management: String,
    encrypt: String,

    initialized: bool,
}

#[cfg(feature = "store")]
#[derive(Default, Debug, Zeroize, ZeroizeOnDrop, Clone)]
pub struct SecretsVault {
    configuration: String,
    monitoring: String,
    management: String,
    encrypt: String,

    key: [u8; 32],
    key_salt: [u8; 32],
    pt_len: usize,
    master: String,

    initialized: bool,
}

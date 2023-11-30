use aes::cipher::block_padding::Pkcs7;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{typenum, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use argon2::Config;
use base64::engine::general_purpose;
use base64::Engine;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::slice::Iter;

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

mod error;

#[derive(Copy, Clone, Debug, Default)]
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
        static SECRETTYPES: [SecretType; 4] = [SecretType::Configuration, SecretType::Monitoring, SecretType::Management, SecretType::Encrypt];
        SECRETTYPES.iter()
    }
}

#[derive(Default, Debug)]
pub struct SecretsVault {
    configuration: Option<String>,
    monitoring: Option<String>,
    management: Option<String>,
    encrypt: Option<String>,

    pub current_secret: SecretType,
    key: [u8; 32],
    key_salt: [u8; 16],
    pt_len: usize,
    master: Option<String>,

    initialized: bool,
}

fn generate_hash(s: &str) -> Result<(Vec<u8>, [u8; 16]), error::VaultError> {
    let mut rng = rand::thread_rng();
    let mut salt: [u8; 16] = [0; 16];
    for s in salt.iter_mut() {
        *s = rng.gen();
    }
    let config = Config::rfc9106_low_mem();
    Ok((argon2::hash_raw(s.as_bytes(), &salt, &config)?, salt))
}

impl SecretsVault {
    pub fn new(master: &str) -> Result<SecretsVault, error::VaultError> {
        let (hash, salt) = generate_hash(master)?;
        let key: [u8; 32] = hash[..32].try_into().unwrap();
        Ok(SecretsVault {
            key,
            pt_len: 48,
            key_salt: salt,
            master: Some(master.to_owned()),
            ..Default::default()
        })
    }

    pub fn create_secret(&self, stype: SecretType, mut value: String) -> Result<(), error::VaultError> {
        while value.len() < 36 {
            value += "0";
        }
        let binding = general_purpose::STANDARD.encode(value.as_bytes());
        let text = binding.as_bytes();

        let mut buf = [0u8; 64];
        buf[..self.pt_len].copy_from_slice(text);
        let mut rng = rand::thread_rng();
        let mut iv: [u8; 16] = [0; 16];
        for x in iv.iter_mut() {
            *x = rng.gen();
        }
        let enc = Aes256CbcEnc::new(&self.key.into(), &iv.into());
        enc.encrypt_padded_mut::<Pkcs7>(&mut buf, self.pt_len)?;
        let mut output = File::create(format!("./vault/vault.{}", stype))?;
        let cipher = [iv.as_slice(), self.key_salt.as_slice(), buf.as_slice()].concat();
        output.write_all(&cipher)?;
        output.flush()?;
        Ok(())
    }

    pub fn read_secret_from_file(&mut self, stype: SecretType) -> Result<(), error::VaultError> {
        let buf = std::fs::read(format!("./vault/vault.{}", stype))?;
        let iv: &GenericArray<u8, typenum::U16> = GenericArray::from_slice(&buf[..16]);
        let salt: &GenericArray<u8, typenum::U16> = GenericArray::from_slice(&buf[16..32]);
        let cipher = &mut buf.clone()[32..];
        match argon2::hash_raw(self.master.as_ref().unwrap().as_bytes(), salt, &Config::rfc9106_low_mem()) {
            Ok(hash) => {
                if !argon2::verify_raw(self.master.as_ref().unwrap().as_bytes(), salt, &hash, &Config::rfc9106_low_mem())? {
                    return Err(error::VaultError::MasterPasswordVerifyError);
                }
                Aes256CbcDec::new(GenericArray::from_slice(&hash), iv).decrypt_padded_mut::<Pkcs7>(cipher)?;
            }
            Err(_) => return Err(error::VaultError::MasterPasswordVerifyError),
        }
        let bytes = String::from_utf8((general_purpose::STANDARD.decode(&cipher[..self.pt_len]))?[..16].to_vec())?;
        match stype {
            SecretType::Configuration => self.configuration = Some(bytes),
            SecretType::Monitoring => self.monitoring = Some(bytes),
            SecretType::Management => self.management = Some(bytes),
            SecretType::Encrypt => self.encrypt = Some(bytes),
        }
        Ok(())
    }

    /// Read all secrets from all vault
    /// After this function execution, Self will contains all secrets
    ///
    /// # Panics
    ///
    /// This function will panic if one of vaults can't be read
    pub fn read_all_secrets(&mut self) {
        for stype in SecretType::iterator() {
            self.read_secret_from_file(*stype).expect(&format!("Can't open vault {stype}"));
        }
        self.initialized = true;
    }

    /// Get the current `SecretType` stored in the Vault
    ///
    /// # Panics
    ///
    /// This function will panic if the current Vault is not initialized correctly
    pub fn get_secret(&self, stype: SecretType) -> &str {
        if !self.initialized {
            panic!("Vault has not yet been initilized");
        }
        match stype {
            SecretType::Configuration => self.configuration.as_deref().unwrap(),
            SecretType::Monitoring => self.monitoring.as_deref().unwrap(),
            SecretType::Management => self.management.as_deref().unwrap(),
            SecretType::Encrypt => self.encrypt.as_deref().unwrap(),
        }
    }
}

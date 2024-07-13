use aes::cipher::block_padding::Pkcs7;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{typenum, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use argon2::Config;
use base64::engine::general_purpose;
use base64::Engine;
use rand::Rng;
use std::fs::File;
use std::io::Write;
use zeroize::Zeroize;

use crate::SecretType;
use crate::{error::VaultError, SecretsVault};

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

fn generate_hash(s: &str) -> Result<(Vec<u8>, [u8; 32]), VaultError> {
    let mut rng = rand::thread_rng();
    let mut salt: [u8; 32] = [0; 32];
    salt.iter_mut().for_each(|s| *s = rng.gen());
    let config = { Config::owasp5() };
    Ok((argon2::hash_raw(s.as_bytes(), &salt, &config)?, salt))
}

impl SecretsVault {
    pub fn new(master: &str) -> Result<SecretsVault, VaultError> {
        let (hash, salt) = generate_hash(master)?;
        let key: [u8; 32] = hash[..32].try_into().unwrap();

        Ok(SecretsVault {
            key,
            pt_len: 48,
            key_salt: salt,
            master: String::from(master),
            configuration: String::new(),
            monitoring: String::new(),
            management: String::new(),
            encrypt: String::new(),
            ..Default::default()
        })
    }

    pub fn create_secret(&self, stype: SecretType, mut value: String) -> Result<(), VaultError> {
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

    pub fn read_secret_from_file(&mut self, stype: SecretType) -> Result<(), VaultError> {
        let buf = std::fs::read(format!("./vault/vault.{}", stype))?;
        let iv: &GenericArray<u8, typenum::U16> = GenericArray::from_slice(&buf[..16]);
        let salt: &GenericArray<u8, typenum::U32> = GenericArray::from_slice(&buf[16..48]);
        let cipher = &mut buf.clone()[48..];
        match argon2::hash_raw(self.master.as_bytes(), salt, &Config::owasp5()) {
            Ok(hash) => {
                if !argon2::verify_raw(self.master.as_bytes(), salt, &hash, &Config::owasp5())? {
                    return Err(VaultError::MasterPasswordVerifyError);
                }
                Aes256CbcDec::new(GenericArray::from_slice(&hash), iv)
                    .decrypt_padded_mut::<Pkcs7>(cipher)?;
            }
            Err(_) => return Err(VaultError::MasterPasswordVerifyError),
        }
        let value = String::from_utf8(
            (general_purpose::STANDARD.decode(&cipher[..self.pt_len]))?[..16].to_vec(),
        )?;
        match stype {
            SecretType::Configuration => self.configuration = value,
            SecretType::Monitoring => self.monitoring = value,
            SecretType::Management => self.management = value,
            SecretType::Encrypt => self.encrypt = value,
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
            self.read_secret_from_file(*stype)
                .unwrap_or_else(|e| panic!("Can't open vault {stype} : {e}"));
        }
        self.initialized = true;
        // We remove the master key from memory there
        self.master.zeroize();
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
            SecretType::Configuration => &self.configuration,
            SecretType::Monitoring => &self.monitoring,
            SecretType::Management => &self.management,
            SecretType::Encrypt => &self.encrypt,
        }
    }
}

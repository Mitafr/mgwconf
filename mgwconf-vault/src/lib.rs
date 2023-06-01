use aes::cipher::block_padding::Pkcs7;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{typenum, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::engine::general_purpose;
use base64::Engine;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::slice::Iter;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

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
    pub configuration: Option<String>,
    pub monitoring: Option<String>,
    pub management: Option<String>,
    pub encrypt: Option<String>,

    pub current_secret: SecretType,
    key: [u8; 16],
    pt_len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeHexError {
    OddLength,
    ParseInt(ParseIntError),
}

impl std::fmt::Display for DecodeHexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DecodeHexError::OddLength => "input string has an odd number of bytes".fmt(f),
            DecodeHexError::ParseInt(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for DecodeHexError {}

impl From<ParseIntError> for DecodeHexError {
    fn from(e: ParseIntError) -> Self {
        DecodeHexError::ParseInt(e)
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, DecodeHexError> {
    if s.len() % 2 != 0 {
        Err(DecodeHexError::OddLength)
    } else {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into())).collect()
    }
}

impl SecretsVault {
    pub fn new(key: &str) -> Result<SecretsVault, DecodeHexError> {
        Ok(SecretsVault {
            key: decode_hex(key)?[..16].try_into().unwrap(),
            pt_len: 48,
            ..Default::default()
        })
    }

    pub fn create_secret(&self, stype: SecretType, mut value: String) {
        while value.len() < 32 {
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
        let enc = Aes128CbcEnc::new(&self.key.into(), &iv.into());
        enc.encrypt_padded_mut::<Pkcs7>(&mut buf, self.pt_len).unwrap();
        let mut output = File::create(format!("./vault/vault.{}", stype)).unwrap();
        let cipher = [iv.as_slice(), buf.as_slice()].concat();
        output.write_all(&cipher).unwrap();
        output.flush().unwrap();
    }

    pub fn read_secret_from_file(&mut self, stype: SecretType) -> Result<(), Box<dyn std::error::Error>> {
        let buf = std::fs::read(format!("./vault/vault.{}", stype))?;
        let iv: &GenericArray<u8, typenum::U16> = GenericArray::from_slice(&buf[..16]);
        let cipher = &mut buf.clone()[16..];
        Aes128CbcDec::new(&self.key.into(), iv).decrypt_padded_mut::<Pkcs7>(cipher).expect("Decrypt Error");
        let utf8 = String::from_utf8(cipher[..self.pt_len].to_vec())?;
        let bytes = String::from_utf8((general_purpose::STANDARD.decode(utf8.as_bytes()))?[..16].to_vec())?;
        match stype {
            SecretType::Configuration => self.configuration = Some(bytes),
            SecretType::Monitoring => self.monitoring = Some(bytes),
            SecretType::Management => self.management = Some(bytes),
            SecretType::Encrypt => self.encrypt = Some(bytes),
        }
        Ok(())
    }

    pub fn read_all_secrets(&mut self) {
        for stype in SecretType::iterator() {
            if let Err(e) = self.read_secret_from_file(*stype) {
                log::error!("{:?}", e);
                return;
            }
        }
    }

    pub fn get_secret(&self, stype: SecretType) -> Option<String> {
        match stype {
            SecretType::Configuration => self.configuration.to_owned(),
            SecretType::Monitoring => self.monitoring.to_owned(),
            SecretType::Management => self.management.to_owned(),
            SecretType::Encrypt => self.encrypt.to_owned(),
        }
    }
}

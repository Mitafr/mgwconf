use std::num::ParseIntError;
use std::slice::Iter;

use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::encode;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

#[derive(Copy, Clone, Debug)]
pub enum SecretType {
    Configuration,
    Monitoring,
    Management,
    Encrypt,
}

impl SecretType {
    pub fn iterator() -> Iter<'static, SecretType> {
        static SECRETTYPES: [SecretType; 4] = [SecretType::Configuration, SecretType::Monitoring, SecretType::Management, SecretType::Encrypt];
        SECRETTYPES.iter()
    }
}

impl Default for SecretType {
    fn default() -> Self {
        SecretType::Configuration
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
    iv: [u8; 16],
    secret_len: u8,
    pt_len: usize,
    buf_len: usize,
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
    pub fn new(key: &str) -> SecretsVault {
        let mut vault = SecretsVault::default();
        vault.key = decode_hex(key).unwrap()[..16].try_into().unwrap();
        vault.iv = [0x2, 0x2, 0x2, 0x2, 0x2, 0x3, 0x3, 0x3, 0x3, 0x3, 0x2, 0x3, 0x2, 0x3, 0x2, 0x3];
        vault.secret_len = 16;
        vault.pt_len = 48;
        vault.buf_len = 64;
        vault
    }

    pub fn create_secret(&self, stype: SecretType, mut value: String) {
        while value.len() < 34 {
            value += "0";
        }
        let binding = encode(value.as_bytes());
        let text = binding.as_bytes();

        let mut buf = [0u8; 64];
        buf[..self.pt_len].copy_from_slice(text);
        Aes128CbcEnc::new(&self.key.into(), &self.iv.into()).encrypt_padded_mut::<Pkcs7>(&mut buf, self.pt_len).unwrap();

        std::fs::write(format!("./vault/vault.{}", stype), buf).unwrap();
    }

    pub fn read_secret_from_file(&mut self, stype: SecretType) {
        let mut buf = std::fs::read(format!("./vault/vault.{}", stype)).unwrap();
        Aes128CbcDec::new(&self.key.into(), &self.iv.into()).decrypt_padded_mut::<Pkcs7>(&mut buf).unwrap();
        let utf8 = String::from_utf8(buf[..self.pt_len].to_vec()).unwrap();
        let bytes = String::from_utf8(base64::decode_config(utf8.as_bytes(), base64::STANDARD).unwrap()[..16].to_vec()).unwrap();
        match stype {
            SecretType::Configuration => self.configuration = Some(bytes),
            SecretType::Monitoring => self.monitoring = Some(bytes),
            SecretType::Management => self.management = Some(bytes),
            SecretType::Encrypt => self.encrypt = Some(bytes),
        }
    }

    pub fn read_all_secrets(&mut self) {
        for stype in SecretType::iterator() {
            self.read_secret_from_file(*stype);
        }
    }
}

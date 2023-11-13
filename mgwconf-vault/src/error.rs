use std::error;
use std::fmt;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum VaultError {
    UnpadError(aes::cipher::block_padding::UnpadError),
    PadError(aes::cipher::inout::PadError),
    Io(std::io::Error),
    FromUtf8Error(FromUtf8Error),
    Base64Error(base64::DecodeError),
    FromHexError(hex::FromHexError),
    Argon2Error(argon2::Error),
    MasterPasswordVerifyError,
}

impl From<argon2::Error> for VaultError {
    fn from(e: argon2::Error) -> Self {
        VaultError::Argon2Error(e)
    }
}

impl From<aes::cipher::block_padding::UnpadError> for VaultError {
    fn from(e: aes::cipher::block_padding::UnpadError) -> Self {
        VaultError::UnpadError(e)
    }
}

impl From<aes::cipher::inout::PadError> for VaultError {
    fn from(e: aes::cipher::inout::PadError) -> Self {
        VaultError::PadError(e)
    }
}

impl From<std::io::Error> for VaultError {
    fn from(e: std::io::Error) -> Self {
        VaultError::Io(e)
    }
}

impl From<FromUtf8Error> for VaultError {
    fn from(e: FromUtf8Error) -> Self {
        VaultError::FromUtf8Error(e)
    }
}

impl From<base64::DecodeError> for VaultError {
    fn from(e: base64::DecodeError) -> Self {
        VaultError::Base64Error(e)
    }
}

impl From<hex::FromHexError> for VaultError {
    fn from(e: hex::FromHexError) -> Self {
        VaultError::FromHexError(e)
    }
}

impl fmt::Display for VaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            VaultError::Argon2Error(e) => ("argon2", e.to_string()),
            VaultError::Base64Error(e) => ("base64", e.to_string()),
            VaultError::UnpadError(e) => ("unpad", e.to_string()),
            VaultError::PadError(e) => ("pad", e.to_string()),
            VaultError::Io(e) => ("IO", e.to_string()),
            VaultError::FromUtf8Error(e) => ("Utf8", e.to_string()),
            VaultError::FromHexError(e) => ("Hex", e.to_string()),
            VaultError::MasterPasswordVerifyError => ("Master password verify", String::from("Can't verify master password")),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl error::Error for VaultError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            VaultError::Argon2Error(e) => e,
            VaultError::Base64Error(e) => e,
            VaultError::UnpadError(_e) => return None,
            VaultError::Io(e) => e,
            VaultError::FromUtf8Error(e) => e,
            VaultError::FromHexError(e) => e,
            VaultError::PadError(_) => return None,
            VaultError::MasterPasswordVerifyError => return None,
        })
    }
}

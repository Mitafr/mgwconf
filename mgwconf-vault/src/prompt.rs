use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use crate::{SecretType, SecretsVault};

fn ask_pwd(stype: SecretType) -> String {
    print!("\x1B[2J\x1B[1;1H");
    let mut s = String::new();
    println!("Pleaser enter {} API KEY", stype);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    s.pop();
    s
}

impl SecretsVault {
    pub fn new(_master: &str) -> Result<SecretsVault, Box<dyn Error>> {
        Ok(SecretsVault {
            configuration: String::new(),
            monitoring: String::new(),
            management: String::new(),
            encrypt: String::new(),
            ..Default::default()
        })
    }

    /// Read all secrets from all vault
    /// After this function execution, Self will contains all secrets
    ///
    /// # Panics
    ///
    /// This function will panic if one of vaults can't be read
    pub fn read_all_secrets(&mut self) {
        for stype in SecretType::iterator() {
            match stype {
                SecretType::Configuration => self.configuration = ask_pwd(SecretType::Configuration),
                SecretType::Monitoring => self.monitoring = ask_pwd(SecretType::Monitoring),
                SecretType::Management => self.management = ask_pwd(SecretType::Management),
                SecretType::Encrypt => self.encrypt = ask_pwd(SecretType::Encrypt),
            }
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
            SecretType::Configuration => &self.configuration,
            SecretType::Monitoring => &self.monitoring,
            SecretType::Management => &self.management,
            SecretType::Encrypt => &self.encrypt,
        }
    }
}

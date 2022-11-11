use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CertificateEntities(pub Vec<CertificateEntity>);

impl CertificateEntities {
    pub fn to_vec_string(&self) -> Vec<String> {
        let mut v = Vec::new();
        for sag in &self.0 {
            v.push(sag.alias.clone());
        }
        v
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CertificateEntity {
    pub alias: String,
    #[serde(rename = "certificateX509")]
    pub certificate_x509: String,
    pub private_key: Option<String>,
}

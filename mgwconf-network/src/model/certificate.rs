pub use super::prelude::*;

#[derive(Serialize, Deserialize, DeriveModel)]
#[mgw_conf(route = "mgw-configuration-api/2.0.0/certificate")]
pub struct CertificateEntities {}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Entities(pub Vec<Entity>);

impl Entities {
    pub fn to_vec_string(&self) -> Vec<String> {
        let mut v = Vec::new();
        for cert in &self.0 {
            v.push(cert.alias.clone());
        }
        v
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, DeriveEntity)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    #[mgw_conf(primary_name)]
    pub alias: String,
    #[serde(rename = "certificateX509")]
    pub certificate_x509: String,
    pub private_key: Option<String>,
}

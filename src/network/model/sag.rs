use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct SagEntities(pub Vec<SagEntity>);

impl SagEntities {
    pub fn to_vec_string(&self) -> Vec<String> {
        let mut v = Vec::new();
        for sag in &self.0 {
            v.push(sag.hostname.clone());
        }
        v
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SagEntity {
    pub hostname: String,
    pub port: usize,
    pub lau_key: Option<String>,
    #[serde(rename = "sslDN")]
    pub ssl_dn: Option<String>,
    pub message_partner_name: Option<String>,
    #[serde(rename = "userDNs")]
    pub user_dns: Vec<String>,
    pub active: bool,
    pub public_certificate_alias: Option<String>,
}

pub use super::prelude::*;

#[derive(Serialize, Deserialize, DeriveModel)]
#[mgw_conf(route = "mgw-configuration-api/2.0.0/sag")]
pub struct SagEntities {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Entities(pub Vec<Entity>);

impl Entities {
    pub fn to_vec_string(&self) -> Vec<String> {
        let mut v = Vec::new();
        for sag in &self.0 {
            v.push(sag.hostname.clone());
        }
        v
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, DeriveEntity)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    #[mgw_conf(primary_name)]
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

impl Default for Entity {
    fn default() -> Self {
        Entity {
            hostname: "test3".to_owned(),
            port: 48002,
            message_partner_name: Some("Sag MP".to_owned()),
            lau_key: Some("Abcd1234Abcd1234Abcd1234Abcd1234".to_owned()),
            ssl_dn: Some("ssl".to_owned()),
            user_dns: vec!["cn=apitest,ou=apicore,o=swhqbebb,o=swift".to_owned()],
            active: true,
            public_certificate_alias: Some("test".to_owned()),
        }
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.hostname, self.port)
    }
}

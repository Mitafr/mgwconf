pub use super::prelude::*;

#[derive(Serialize, Deserialize, DeriveModel, Debug)]
#[mgw_conf(route = "mgw-configuration-api/2.0.0/sag")]
pub struct SagEntities {}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
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
    #[mgw_conf(primary_name, delete_query_attr("hostname"))]
    pub hostname: String,
    #[mgw_conf(delete_query_attr("port"))]
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
        use rand::prelude::*;
        let rng = rand::thread_rng().gen_range(0..100);
        Entity {
            hostname: format!("test{}", rng),
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

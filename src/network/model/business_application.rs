pub use super::prelude::*;

#[derive(Serialize, Deserialize, DeriveModel)]
#[mgw_conf(route = "mgw-configuration-api/2.0.0/business-application")]
pub struct BusinessApplications {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Entities(pub Vec<Entity>);

impl Entities {
    pub fn to_vec_string(&self) -> Vec<String> {
        let mut v = Vec::new();
        for business in &self.0 {
            v.push(business.application_name.clone());
        }
        v
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, DeriveEntity)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    #[mgw_conf(primary_name)]
    pub application_name: String,
    pub shared_secret: String,
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.application_name, self.shared_secret)
    }
}

pub use super::prelude::*;

#[derive(Serialize, Deserialize, DeriveModel)]
#[mgw_conf(route = "mgw-configuration-api/2.0.0/profile")]
pub struct Profiles {}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Entities(pub Vec<Entity>);

impl Entities {
    pub fn to_vec_string(&self) -> Vec<String> {
        let mut v = Vec::new();
        for profile in &self.0 {
            v.push(profile.application_name.to_owned());
        }
        v
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, DeriveEntity)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    #[mgw_conf(primary_name)]
    pub application_name: String,
    pub profile_name: String,
    pub tracker_profile: String,
    pub rbac_scope: String,
    pub user_dns: Vec<String>,
}

pub use super::prelude::*;

#[derive(Serialize, Deserialize, Debug, DeriveModel)]
pub struct BusinessApplications {
    pub route: String,
}

impl Default for BusinessApplications {
    fn default() -> BusinessApplications {
        Self {
            route: "mgw-configuration-api/2.0.0/business-application".to_owned(),
        }
    }
}

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

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub application_name: String,
    pub shared_secret: String,
}

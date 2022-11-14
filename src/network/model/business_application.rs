use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BusinessApplications(pub Vec<BusinessApplication>);

impl BusinessApplications {
    pub fn to_vec_string(&self) -> Vec<String> {
        let mut v = Vec::new();
        for business_app in &self.0 {
            v.push(business_app.application_name.clone());
        }
        v
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BusinessApplication {
    pub application_name: String,
    pub shared_secret: String,
}

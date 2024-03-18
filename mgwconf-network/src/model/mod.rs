use std::any::Any;

use self::configuration::SagEntity;

pub mod configuration;
pub mod management;
pub mod monitoring;

///
/// Contains the entity itself
///
pub trait InnerEntityTrait: std::fmt::Debug + Send + Sync {
    fn entity_type(&self) -> &str;
    fn name(&self) -> &str;
    fn to_string(&self) -> String {
        String::new()
    }
    fn as_any(&self) -> &dyn Any;
}

///
/// Contains a collection of entities
///
pub trait CollectionEntityTrait: std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;

    fn get(&self, index: usize) -> Option<Box<dyn InnerEntityTrait>>;
}

impl Default for SagEntity {
    fn default() -> Self {
        SagEntity {
            hostname: String::from("test3"),
            port: 48002,
            message_partner_name: Some(String::from("SAG MP")),
            user_dns: vec![String::from("cn=apitest,ou=apicore,o=agrifrpp,o=swift")],
            lau_key: Some(String::from("Abcd1234Abcd1234Abcd1234Abcd1234")),
            ssl_dn: Some(String::from("ss")),
            active: Some(false),
            public_certificate_alias: Some(String::from("test")),
        }
    }
}

#![allow(unused_imports)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;

pub mod apis;
pub mod models;

/// Contains the entity itself
pub trait InnerEntityTrait: std::fmt::Debug + Send + Sync {
    fn entity_type(&self) -> &str;
    fn name(&self) -> String;
    fn to_string(&self) -> String {
        String::new()
    }
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Contains a collection of entities
pub trait CollectionEntityTrait: std::fmt::Debug {
    fn as_any(&self) -> &dyn std::any::Any;

    fn get(&self, index: usize) -> Option<Box<dyn InnerEntityTrait>>;
}

use std::any::Any;

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

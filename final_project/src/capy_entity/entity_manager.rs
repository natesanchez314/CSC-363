use crate::entity::Entity;
use crate::entity::EntityName;

pub trait EntityManager {
    fn add(&mut self, entity: Box<dyn Entity>);
    fn pop(&mut self) -> Option<Box<dyn Entity>>;
    fn remove(&mut self, entity: EntityName) -> Option<Box<dyn Entity>>;
}
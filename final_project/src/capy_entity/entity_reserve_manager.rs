use crate::entity::Entity;
use crate::entity::EntityName;
use crate::capy_entity::entity_manager::EntityManager;

pub struct EntityReserveManager {
    pub entities: Vec<Box<dyn Entity>>
}

impl EntityManager for EntityReserveManager {
    fn add(&mut self, entity: Box<dyn Entity>) {
        self.entities.push(entity);
    }

    fn pop(&mut self) -> Option<Box<dyn Entity>> {
        self.entities.pop()
    }

    fn remove(&mut self, _entity: EntityName) -> Option<Box<dyn Entity>> {
        // TODO
        self.pop()
    }
}

impl EntityReserveManager {
    pub fn new() -> Self {
        Self { entities: Vec::new() }
    }
}
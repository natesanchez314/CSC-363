use notan::draw::Draw;
use notan::math::Vec2;

use crate::entity::Entity;
use crate::entity::EntityName;
use crate::capy_entity::entity_manager::EntityManager;

pub struct EntityActiveManager {
    pub entities: Vec<Box<dyn Entity>>
}

impl EntityManager for EntityActiveManager {
    fn add(&mut self, entity: Box<dyn Entity>) {
        self.entities.push(entity);
    }

    fn pop(&mut self) -> Option<Box<dyn Entity>> {
        self.entities.pop()
    }

    fn remove(&mut self, _entity: EntityName) -> Option<Box<dyn Entity>> {
        self.pop()
    }
}

impl EntityActiveManager {
    pub fn new() -> Self {
        Self { entities: Vec::with_capacity(20) }
    }

    pub fn update(&mut self, delta: f32) {
        for entity in self.entities.iter_mut() {
            entity.update(delta);
        }
    }

    pub fn draw(&self, draw: &mut Draw) {
        for entity in self.entities.iter() {
            entity.draw(draw);
        }
    }

    pub fn collisions(&mut self, delta: f32) {
        let l = self.entities.len();
        let mut forces: Vec<(Vec2, Vec2)> = Vec::with_capacity(l);
        // Check for collision and get forces we need
        for i in 0..l {
            forces.push((Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)));
            for j in 0..l {
                if i != j {
                    let force = self.entities[i].collide(&self.entities[j], delta);
                    forces[i].0 += force.0;
                    forces[i].1 -= force.1;
                }
            }
        }
        for (i, entity) in self.entities.iter_mut().enumerate() {
            // Collision resolution
            entity.apply_force(forces[i].0);
            if forces[i].0.x != 0.0 || forces[i].0.y != 0.0 {
                entity.set_colliding(true);
            } else {
                entity.set_colliding(false);
            }
            // Position correction
            entity.set_position(entity.get_position() + forces[i].1);
        }
    }
}
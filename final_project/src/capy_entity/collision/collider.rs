use crate::capy_entity::entity::Entity;

use super::{aabb::AABB, circle_collider::CircleCollider, manifold::Manifold};

pub trait Collidable: Entity {
    fn is_colliding(&self) -> bool;
    fn set_size(&mut self, x_size: f32, y_size: f32);

    // These collider functions are to check for collision
    fn check_collision(&self, other: &dyn Collidable) -> (bool, Manifold);
    fn check_collision_box(&self, other: &Box<dyn Collidable>) -> (bool, Manifold);
    fn check_collision_aabb(&self, other: &AABB) -> (bool, Manifold);
    fn check_collision_circle(&self, other: &CircleCollider) -> (bool, Manifold);
}
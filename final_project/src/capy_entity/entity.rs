use notan::{draw::Draw, math::Vec2};

use super::collision::{aabb::AABB, manifold::Manifold};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum EntityName {
    CircleCollider,
    AabbCollider,
    Ball,
    Square,
    Floor
}

pub trait Entity {
    fn update(&mut self, delta: f32);
    fn draw(&self, draw: &mut Draw);
    fn get_name(&self) -> EntityName;
    fn collide(&self, other: &Box<dyn Entity>, delta: f32) -> (Vec2, Vec2);
    fn apply_force(&mut self, force: Vec2);
    fn check_bounding_boxes(&self, other_collider: &AABB) -> (bool, Manifold);
    fn get_velocity(&self) -> Vec2;
    fn get_restitution(&self) -> f32;
    fn get_mass(&self) -> f32;
    fn get_inv_mass(&self) -> f32;
    fn get_position(&self) -> Vec2;
    fn set_position(&mut self, pos: Vec2);
    fn set_colliding(&mut self, b: bool);
}
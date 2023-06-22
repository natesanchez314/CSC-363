use std::rc::Rc;

use notan::draw::Draw;
use notan::math::Vec2;

use crate::entity::*;

use crate::capy_gfx::capy_texture::CapyTexture;
use crate::capy_entity::collision::collider::Collidable;

use super::collision::aabb::AABB;
use super::collision::manifold::Manifold;

// Collidable but does not move 
pub struct StaticEntity {
    pub name: EntityName,
    pub position: Vec2,
    pub size: Vec2,
    pub should_draw: bool,
    pub texture: Option<Rc<dyn CapyTexture>>,
    pub collider: Option<AABB>,
    pub restitution: f32,
    pub mass: f32,
    pub inv_mass: f32
}

impl Entity for StaticEntity {
    fn update(&mut self, delta: f32) {
        match &mut self.collider {
            Some(c) => {
                c.set_position(self.position);
                c.set_size(self.size.x, self.size.y);
                c.update(delta);
            }
            None => {
                println!("No collider...");
            }
        }
    }

    fn draw(&self, draw: &mut Draw) {
        if self.should_draw {
            match &self.texture {
                Some(t) => {
                    t.draw(draw, self.position.x, self.position.y, self.size);
                }
                None => {}
            }
        }
        match &self.collider {
            Some(c) => {
                c.draw(draw);
            }
            None => {}
        }
    }

    fn collide(&self, other: &Box<dyn Entity>, _delta: f32) -> (Vec2, Vec2) {
        match &self.collider {
            Some(c) => {
                if other.check_bounding_boxes(c).0 {
                    return (Vec2{ x: 0.01, y: 0.0 }, Vec2{ x: 0.0, y: 0.0 });
                }
            }
            None => {}
        }
        (Vec2{ x: 0.0, y: 0.0 }, Vec2{ x: 0.0, y: 0.0 })
    }

    fn check_bounding_boxes(&self, other_collider: &AABB) -> (bool, Manifold) {
        match &self.collider {
            Some(c) => {
                // Are AABBS colliding
                if c.max.x < other_collider.min.x || c.min.x > other_collider.max.x {
                    return (false, Manifold{penetration: 0.0, normal: Vec2{x: 0.0, y: 0.0}, n: Vec2{x: 0.0, y: 0.0}});
                }
                if c.max.y < other_collider.min.y || c.min.y > other_collider.max.y {
                    return (false, Manifold{penetration: 0.0, normal: Vec2{x: 0.0, y: 0.0}, n: Vec2{x: 0.0, y: 0.0}});
                }
                // Check if we have an inner collider
                match &c.inner_collider {
                    Some(i_c) => {
                        match &other_collider.inner_collider {
                            Some(o_i_c) => {
                                // Both have inner collider
                                //i_c.check_collision_box(o_i_c)
                                o_i_c.check_collision_box(i_c)
                            }
                            None => {
                                // LHS has inner collider
                                other_collider.check_collision_box(i_c)
                            }
                        }
                    }
                    None => {
                        match &other_collider.inner_collider {
                            Some(o_i_c) => {
                                // RHS has inner collider
                                o_i_c.check_collision_aabb(c)
                            }
                            None => {
                                // Neither have inner collider
                                other_collider.check_collision_aabb(c)
                            }
                        }
                    }
                }
            }
            None => {
                (false, Manifold{penetration: 0.0, normal: Vec2{x: 0.0, y: 0.0}, n: Vec2{x: 0.0, y: 0.0}})
            }
        }
    }

    fn apply_force(&mut self, _force: Vec2) { }

    fn get_name(&self) -> EntityName { self.name }

    fn get_velocity(&self) -> Vec2 { Vec2{x: 0.0, y: 0.0} }

    fn get_restitution(&self) -> f32 { self.restitution }

    fn get_mass(&self) -> f32 { self.mass }

    fn get_inv_mass(&self) -> f32 { self.inv_mass }

    fn get_position(&self) -> Vec2 { self.position }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    fn set_colliding(&mut self, b: bool) {
        match self.collider {
            Some(ref mut c) => {
                c.set_colliding(b);
            }
            None => {}
        }
    }
}
use std::rc::Rc;

use notan::math::Vec2;
use notan::graphics::color::Color;

use crate::{
    capy_entity::{
        collision::collider::Collidable,
        entity::Entity,
        entity::EntityName
    }, 
    capy_gfx::circle_texture::CircleTexture
};

use super::{aabb::AABB, manifold::Manifold};

pub struct CircleCollider {
    pub radius: f32,
    pub position: Vec2,
    pub texture: Option<Rc<CircleTexture>>,
    pub should_draw: bool,
    pub is_colliding: bool,
}

impl Collidable for CircleCollider {
    fn is_colliding(&self) -> bool {
        self.is_colliding
    }

    fn set_size(&mut self, x_size: f32, _y_size: f32) {
        self.radius = x_size;
    }

    fn check_collision(&self, other: &dyn Collidable) -> (bool, Manifold) {
        other.check_collision_circle(self)
    }

    fn check_collision_box(&self, other: &Box<dyn Collidable>) -> (bool, Manifold) {
        other.check_collision_circle(self)
    }

    fn check_collision_aabb(&self, other: &AABB) -> (bool, Manifold) {
        let mut m = Manifold{penetration: 0.0, normal: Vec2 { x: 0.0, y: 0.0 }, n: Vec2{x: 0.0, y: 0.0}};
        let a_pos = self.get_position();
        let b_pos = other.get_position();
        let n = b_pos - a_pos;
        let mut closest = n;
        let x_extent = (other.max.x - other.min.x) / 2.0;
        let y_extent = (other.max.y - other.min.y) / 2.0;
        closest.x = closest.x.clamp(-x_extent, x_extent);
        closest.y = closest.y.clamp(-y_extent, y_extent);

        let mut inside = false;

        if n == closest {
            inside = true;
            if n.x.abs() < n.y.abs() {
                if closest.x > 0.0 {
                    closest.x = x_extent;
                } else {
                    closest.x = -x_extent;
                }
            } else {
                if closest.y > 0.0 {
                    closest.y = y_extent;
                } else {
                    closest.y = -y_extent;
                }
            }
        }

        let normal = closest - n;
        let mut d = normal.length_squared();
        let r = self.radius;

        if d > r * r && !inside {
            return (false, m);
        }
        if self.position.x > other.min.x && self.position.x < other.max.x && self.position.y < other.get_position().y {
            m.normal.x = 0.0;
            m.normal.y = 1.0;
            m.penetration = r - d;
            m.n = n;
        } else {
            d = d.sqrt();
            if inside {
                m.normal = n;
                m.penetration = r - d;
                m.n = n;
            } else {
                m.normal = -n;
                m.penetration = r - d;
                m.n = n;
            }
            m.normal = m.normal.normalize()
        }
        (true, m)
    }

    fn check_collision_circle(&self, other: &CircleCollider) -> (bool, Manifold) {
        let r = (self.radius + other.radius) * (self.radius + other.radius);
        let n = self.position - other.get_position();
        let colliding = r < n.length_squared();
        
        let mut m = Manifold{penetration: 0.0, normal: Vec2 { x: 0.0, y: 0.0 }, n: Vec2{x: 0.0, y: 0.0}};
        if colliding {
            let distance = n.length();
            if distance != 0.0 {
                m.penetration = distance - r;

                //todo!("There was a t here and I'm not sure what that was...");
                //let t = d -
                m.normal = n / distance;
                m.n = -n;
            } else {
                m.penetration = self.radius;
                m.normal.x = 1.0;
                m.normal.y = 0.0;
            }
        }
        (colliding, m)
    }
}

impl Entity for CircleCollider {
    fn update(&mut self, _delta: f32) { }

    fn draw(&self, draw: &mut notan::draw::Draw) {
        if self.should_draw {
            match &self.texture {
                Some(t) => {
                    if self.is_colliding {
                        t.draw_circle(draw, self.position, self.radius, Color::RED);
                    } else {
                        t.draw_circle(draw, self.position, self.radius, Color::GREEN);
                    }
                }
                None => {}
            }
        }
    }

    fn collide(&self, _other: &Box<dyn Entity>, _delta: f32) -> (Vec2, Vec2) { (Vec2{ x: 0.0, y: 0.0 }, Vec2{ x: 0.0, y: 0.0 }) }

    fn check_bounding_boxes(&self, _collider: &AABB) -> (bool, Manifold) { (false, Manifold{penetration: 0.0, normal: Vec2{x: 0.0, y: 0.0}, n: Vec2{x: 0.0, y: 0.0} }) }

    fn apply_force(&mut self, _force: Vec2) {}

    fn get_name(&self) -> crate::capy_entity::entity::EntityName { EntityName::CircleCollider }

    fn get_velocity(&self) -> Vec2 { Vec2{x: 0.0, y: 0.0} }

    fn get_restitution(&self) -> f32 { 0.0 }

    fn get_position(&self) -> Vec2 { self.position }

    fn get_mass(&self) -> f32 { 0.0 }

    fn get_inv_mass(&self) -> f32 { 0.0 }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos
    }

    fn set_colliding(&mut self, b: bool) {
        self.is_colliding = b;
    }
}
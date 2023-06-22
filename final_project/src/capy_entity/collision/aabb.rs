use std::rc::Rc;

use notan::{math::Vec2, graphics::color::Color};

use crate::{
    capy_entity::{
        collision::collider::Collidable,
        entity::Entity,
        entity::EntityName
    }, capy_gfx::rect_texture::RectTexture,
};

use super::{circle_collider::CircleCollider, manifold::Manifold};

pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
    pub texture: Option<Rc<RectTexture>>,
    pub should_draw: bool,
    pub inner_collider: Option<Box<dyn Collidable>>,
    pub is_colliding: bool
}

impl AABB {
}

impl Collidable for AABB {
    fn is_colliding(&self) -> bool {
        self.is_colliding
    }

    fn set_size(&mut self, x_size: f32, y_size: f32) {
        self.max.x = 2.0 * x_size + self.min.x;
        self.max.y = 2.0 * y_size + self.min.y;
        match &mut self.inner_collider {
            Some(ic) => {
                ic.set_size(x_size, y_size)
            }
            None => {}
        }
    }

    fn check_collision(&self, other: &dyn Collidable) -> (bool, Manifold) {
        other.check_collision_aabb(self)
    }

    fn check_collision_box(&self, other: &Box<dyn Collidable>) -> (bool, Manifold) {
        other.check_collision_aabb(self)
    }

    fn check_collision_aabb(&self, other: &AABB) -> (bool, Manifold) {
        let mut colliding = false;
        let mut m = Manifold{penetration: 0.0, normal: Vec2 { x: 0.0, y: 0.0 }, n: Vec2{x: 0.0, y: 0.0}};
        let n = other.get_position() - self.get_position();
        let a_x_extent = (self.max.x - self.min.x) / 2.0;
        let b_x_extent = (other.max.x - other.min.x) / 2.0;
        let x_overlap = a_x_extent + b_x_extent - n.x.abs();
        if x_overlap > 0.0 {
            let a_y_extent = (self.max.y - self.min.y) / 2.0;
            let b_y_extent = (other.max.y - other.min.y) / 2.0;
            let y_overlap = a_y_extent + b_y_extent - n.y.abs();
            if y_overlap > 0.0 {
                if x_overlap < y_overlap {
                    if n.x < 0.0 {
                        m.normal.x = -1.0;
                        m.normal.y = 0.0;
                    } else {
                        m.normal.x = 0.0;
                        m.normal.y = 0.0;
                    }
                    m.penetration = x_overlap;
                    colliding = true;
                } else {
                    if n.y < 0.0 {
                        m.normal.x = 0.0;
                        m.normal.y = 1.0;
                    } else {
                        m.normal.x = 0.0;
                        m.normal.y = 1.0;
                    }
                    m.penetration = y_overlap;
                    colliding = true;
                }
            }
        }
        m.n = n;
        (colliding, m)
    }

    fn check_collision_circle(&self, other: &CircleCollider) -> (bool, Manifold) {
        let mut m = Manifold{penetration: 0.0, normal: Vec2 { x: 0.0, y: 0.0 }, n: Vec2{x: 0.0, y: 0.0}};
        let a_pos = self.get_position();
        let b_pos = other.get_position();
        let n = b_pos - a_pos;
        let mut closest = n;
        let x_extent = (self.max.x - self.min.x) / 2.0;
        let y_extent = (self.max.y - self.min.y) / 2.0;
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

        let normal = n - closest;
        let mut d = normal.length_squared();
        let r = other.radius;

        if d > r * r && !inside {
            return (false, m);
        }
        d = d.sqrt();
        if inside {
            m.normal = n;
            m.penetration = r - d;
        } else {
            m.normal = -n;
            m.penetration = r - d;
        }
        m.n = n;
        (true, m)
    }
}

impl Entity for AABB {
    fn update(&mut self, _delta: f32) { }

    fn draw(&self, draw: &mut notan::draw::Draw) { 
        if self.should_draw {
            match &self.texture {
                Some(t) => {
                    let x_size = self.max.x - self.min.x;
                    let y_size = self.max.y - self.min.y;
                    if self.is_colliding {
                        t.draw_rect(draw, self.min, Vec2::new(x_size, y_size), Color::RED);
                    } else {
                        t.draw_rect(draw, self.min, Vec2::new(x_size, y_size), Color::GREEN);
                    }
                }
                None => {}
            }
            match &self.inner_collider {
                Some(ic) => {
                    ic.draw(draw);
                }
                None => {}
            }
        }
    }

    fn collide(&self, _other: &Box<dyn Entity>, _delta: f32) -> (Vec2, Vec2) { (Vec2{ x: 0.0, y: 0.0 }, Vec2{ x: 0.0, y: 0.0 }) }

    fn check_bounding_boxes(&self, _collider: &AABB) -> (bool, Manifold) { (false, Manifold{penetration: 0.0, normal: Vec2{x: 0.0, y: 0.0}, n: Vec2{x: 0.0, y: 0.0}}) }

    fn apply_force(&mut self, _force: Vec2) {}

    fn get_name(&self) -> crate::capy_entity::entity::EntityName { EntityName::AabbCollider }

    fn get_velocity(&self) -> Vec2 { Vec2{x: 0.0, y: 0.0} }

    fn get_restitution(&self) -> f32 { 0.0 }

    fn get_mass(&self) -> f32 { 0.0 }

    fn get_inv_mass(&self) -> f32 { 0.0 }

    fn get_position(&self) -> Vec2 { 
        Vec2{ 
            x: self.min.x + (self.max.x - self.min.x) / 2.0,
            y: self.min.y + (self.max.y - self.min.y) / 2.0
        } 
    }

    fn set_position(&mut self, pos: Vec2) {        
        let x_pos = self.min.x + (self.max.x - self.min.x) / 2.0;
        let y_pos = self.min.y + (self.max.y - self.min.y) / 2.0;
        let x_offset = pos.x - x_pos;
        let y_offset = pos.y - y_pos;

        self.min.x += x_offset;
        self.min.y += y_offset;
        match &mut self.inner_collider {
            Some(ic) => {
                ic.set_position(pos)
            }
            None => {}
        }
    }

    fn set_colliding(&mut self, b: bool) {
        self.is_colliding = b;
        match self.inner_collider {
            Some(ref mut c) => {
                c.set_colliding(b);
            }
            None => {}
        }
    }
}
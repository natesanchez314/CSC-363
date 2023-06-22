use notan::{draw::Draw, math::Vec2};

#[derive(PartialEq, Eq, Hash)]
pub enum TextureName {
    Circle,
    Rect,
    //CircleCollider,
    //AabbCollider
}

pub trait CapyTexture {
    fn draw(&self, draw: &mut Draw, x: f32, y:f32, size: Vec2);
}
use notan::math::Vec2;

pub struct Manifold {
    pub penetration: f32,
    pub normal: Vec2,
    pub n: Vec2,
}
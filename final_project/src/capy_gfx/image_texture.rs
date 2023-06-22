use notan::graphics::Texture;
use notan::draw::*;
use notan::math::Vec2;

use super::capy_texture::*;

pub struct ImageTexture  {
    pub texture: Texture
}

impl CapyTexture for ImageTexture {
    fn draw(&self, draw: &mut Draw, x: f32, y:f32, size: Vec2) {
        draw.image(&self.texture).position(x, y).size(size.x, size.y);
    }
}
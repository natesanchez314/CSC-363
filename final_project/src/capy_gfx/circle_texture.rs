use notan::graphics::color::Color;
use notan::draw::*;
use notan::math::Vec2;

use super::capy_texture::*;

pub struct CircleTexture  {
    pub fill_color: Color,
    pub stroke_color: Color,
    pub stroke: f32,
    pub fill: bool
}

impl CircleTexture {
    pub fn draw_circle(&self, draw: &mut Draw, pos: Vec2, rad: f32, color: Color) {
        if self.fill {
            draw.circle(rad)
                .position(pos.x, pos.y)
                .fill_color(self.fill_color)
                .fill()
                .stroke_color(color)
                .stroke(self.stroke);
        } else {
            draw.circle(rad)
                .position(pos.x, pos.y)
                .stroke_color(color)
                .stroke(self.stroke);
        }
    }
}

impl CapyTexture for CircleTexture {
    fn draw(&self, draw: &mut Draw, x: f32, y:f32, size: Vec2) {
        if self.fill {
            draw.circle(size.x)
                .position(x, y)
                .fill_color(self.fill_color)
                .fill()
                .stroke_color(self.stroke_color)
                .stroke(self.stroke);
        } else {
            draw.circle(size.x)
                .position(x, y)
                .stroke_color(self.stroke_color)
                .stroke(self.stroke);
        }
    }
}
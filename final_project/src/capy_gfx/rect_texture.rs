use notan::graphics::color::Color;
use notan::draw::*;
use notan::math::Vec2;

use super::capy_texture::*;

pub struct RectTexture {
    pub fill_color: Color,
    pub stroke_color: Color,
    pub stroke: f32,
    pub fill: bool
}

impl RectTexture {
    pub fn draw_rect(&self, draw: &mut Draw, min: Vec2, max: Vec2, color: Color) {
        if self.fill {
            draw.rect(min.into(), max.into())
                .fill_color(self.fill_color)
                .fill()
                .stroke_color(color)
                .stroke(self.stroke);
        } else {
            draw.rect(min.into(), max.into())
                .stroke_color(color)
                .stroke(self.stroke);
        }
    }
}

impl CapyTexture for RectTexture {
    fn draw(&self, draw: &mut Draw, x: f32, y:f32, size: Vec2) {
        if self.fill {
            draw.rect((x - size.x, y - size.y), (size.x * 2.0, size.y * 2.0))
                .fill_color(self.fill_color)
                .fill()
                .stroke_color(self.stroke_color)
                .stroke(self.stroke);
        } else {
            draw.rect((x, y), (size.x, size.y))
                .stroke_color(self.stroke_color)
                .stroke(self.stroke);
        }
    }
}
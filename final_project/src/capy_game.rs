use notan::prelude::*;
use notan::draw::*;

use crate::capy_entity::entity_active_manager::EntityActiveManager;
use crate::capy_entity::entity_reserve_manager::EntityReserveManager;

use crate::capy_gfx::texture_manager::TextureManager;

#[derive(AppState)]
pub struct CapyGame {
    pub texture_manager: TextureManager,
    pub entity_active_manager: EntityActiveManager,
    pub entity_reserve_manager: EntityReserveManager,
    pub mouse_pos_x: f32,
    pub mouse_pos_y: f32
}

impl CapyGame {
    pub fn new() -> Self {
        Self {
            texture_manager: TextureManager::new(),
            entity_active_manager: EntityActiveManager::new(),
            entity_reserve_manager: EntityReserveManager::new(),
            mouse_pos_x: 0.0,
            mouse_pos_y: 0.0
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.entity_active_manager.collisions(delta);
        self.entity_active_manager.update(delta);
    }

    pub fn draw(&mut self, gfx: &mut Graphics, draw: &mut Draw) {
        draw.clear(Color::BLACK);
        self.entity_active_manager.draw(draw);
        gfx.render(draw);
    }
}
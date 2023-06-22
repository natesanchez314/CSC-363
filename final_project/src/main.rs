use std::rc::Rc;

use capy_entity::collision::aabb::AABB;
use capy_entity::entity;
use capy_gfx::rect_texture::RectTexture;
use notan::prelude::*;
use notan::draw::*;
use notan::math::Vec2;

mod capy_game;
mod capy_gfx;
mod capy_entity;

use capy_game::CapyGame;
use capy_gfx::capy_texture::TextureName;
use capy_gfx::circle_texture::CircleTexture;
use capy_entity::physics_entity::PhysicsEntity;
use capy_entity::static_entity::StaticEntity;
use capy_entity::entity::EntityName;
use capy_entity::entity_manager::EntityManager;
use capy_entity::collision::circle_collider::CircleCollider;

use crate::capy_entity::entity::Entity;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
        
}

fn init(_gfx: &mut Graphics) -> CapyGame {
    let mut capy_game = CapyGame::new();

    let draw_colliders = false;
    let init_gravity = 1.0;

    let circle_texture = Rc::new(CircleTexture {
        fill_color: Color::BLUE,
        stroke_color: Color::RED,
        stroke: 0.0,
        fill: true
    });

    let collider_circle_texture = Rc::new(CircleTexture {
        fill_color: Color::BLUE,
        stroke_color: Color::GREEN,
        stroke: 2.0,
        fill: false
    });

    let rect_texture = Rc::new(RectTexture {
        fill_color: Color::AQUA,
        stroke_color: Color::RED,
        stroke: 1.0,
        fill: true
    });

    let collider_rect_texture = Rc::new(RectTexture {
        fill_color: Color::RED,
        stroke_color: Color::GREEN,
        stroke: 2.0,
        fill: false
    });

    capy_game.texture_manager.add(TextureName::Circle, circle_texture);
    capy_game.texture_manager.add(TextureName::Rect, rect_texture);

    let ball = Box::new(PhysicsEntity {
        name: EntityName::Ball,
        position: Vec2::new(200.0, 100.0),
        velocity: Vec2::new(0.005, init_gravity),
        size: Vec2::new(20.0, 20.0),
        should_draw: true,
        texture: capy_game.texture_manager.find(TextureName::Circle),
        collider: Some(AABB{
            min: Vec2::new(250.0, 250.0),
            max: Vec2::new(270.0, 270.0),
            texture: Some(collider_rect_texture.clone()),
            should_draw: draw_colliders,
            is_colliding: false,
            inner_collider: Some(Box::new(CircleCollider {
                radius: 20.0,
                position: Vec2::new(250.0, 250.0),
                texture: Some(collider_circle_texture.clone()),
                should_draw: draw_colliders,
                is_colliding: false
            }))
        }),
        restitution: 0.5,
        mass: 5.0,
        inv_mass: 1.0 / 5.0
    });

    let ball2 = Box::new(PhysicsEntity {
        name: EntityName::Ball,
        position: Vec2::new(300.0, 200.0),
        velocity: Vec2::new(0.0, init_gravity),
        size: Vec2::new(10.0, 10.0),
        should_draw: true,
        texture: capy_game.texture_manager.find(TextureName::Circle),
        collider: Some(AABB{
            min: Vec2::new(300.0, 200.0),
            max: Vec2::new(300.0, 350.0),
            texture: Some(collider_rect_texture.clone()),
            should_draw: draw_colliders,
            is_colliding: false,
            inner_collider: Some(Box::new(CircleCollider {
                radius: 20.0,
                position: Vec2::new(300.0, 200.0),
                texture: Some(collider_circle_texture.clone()),
                should_draw: draw_colliders,
                is_colliding: false
            }))
        }),
        restitution: 0.5,
        mass: 3.0,
        inv_mass: 1.0 / 3.0
    });

    let square = Box::new(PhysicsEntity {
        name: EntityName::Square,
        position: Vec2::new(225.0, 225.0),
        velocity: Vec2::new(0.0, init_gravity),
        size: Vec2::new(20.0, 20.0),
        should_draw: true,
        texture: capy_game.texture_manager.find(TextureName::Rect),
        collider: Some(AABB{
            min: Vec2::new(300.0, 200.0),
            max: Vec2::new(250.0, 250.0),
            texture: Some(collider_rect_texture.clone()),
            should_draw: draw_colliders,
            inner_collider: None,
            is_colliding: false
        }),
        restitution: 0.5,
        mass: 5.0,
        inv_mass: 1.0 / 5.0
    });

    let floor = Box::new(StaticEntity {
        name: EntityName::Floor,
        position: Vec2::new(400.0, 600.0),
        size: Vec2::new(400.0, 50.0),
        should_draw: true,
        texture: capy_game.texture_manager.find(TextureName::Rect),
        collider: Some(AABB{
            min: Vec2::new(0.0, 620.0),
            max: Vec2::new(800.0, 600.0),
            texture: Some(collider_rect_texture.clone()),
            should_draw: draw_colliders,
            inner_collider: None,
            is_colliding: false
        }),
        restitution: 0.5,
        mass: 100.0,
        inv_mass: 1.0 / 100.0
    });

    let ball3 = Box::new(StaticEntity {
        name: EntityName::Ball,
        position: Vec2::new(400.0, 300.0),
        size: Vec2::new(30.0, 30.0),
        should_draw: true,
        texture: capy_game.texture_manager.find(TextureName::Circle),
        collider: Some(AABB{
            min: Vec2::new(250.0, 250.0),
            max: Vec2::new(270.0, 270.0),
            texture: Some(collider_rect_texture.clone()),
            should_draw: draw_colliders,
            is_colliding: false,
            inner_collider: Some(Box::new(CircleCollider {
                radius: 20.0,
                position: Vec2::new(250.0, 250.0),
                texture: Some(collider_circle_texture.clone()),
                should_draw: draw_colliders,
                is_colliding: false
            }))
        }),
        restitution: 0.4,
        mass: 5.0,
        inv_mass: 1.0 / 5.0
    });

    capy_game.entity_active_manager.add(ball);
    capy_game.entity_active_manager.add(ball2);
    capy_game.entity_active_manager.add(square);
    capy_game.entity_active_manager.add(floor);
    capy_game.entity_active_manager.add(ball3);

    for _i in 0..10 {
        capy_game.entity_reserve_manager.add(Box::new(PhysicsEntity {
            name: EntityName::Ball,
            position: Vec2::new(200.0, 100.0),
            velocity: Vec2::new(0.0, init_gravity),
            size: Vec2::new(20.0, 20.0),
            should_draw: true,
            texture: capy_game.texture_manager.find(TextureName::Circle),
            collider: Some(AABB{
                min: Vec2::new(250.0, 250.0),
                max: Vec2::new(270.0, 270.0),
                texture: Some(collider_rect_texture.clone()),
                should_draw: draw_colliders,
                is_colliding: false,
                inner_collider: Some(Box::new(CircleCollider {
                    radius: 20.0,
                    position: Vec2::new(250.0, 250.0),
                    texture: Some(collider_circle_texture.clone()),
                    should_draw: draw_colliders,
                    is_colliding: false
                }))
            }),
            restitution: 0.4,
            mass: 5.0,
            inv_mass: 1.0 / 5.0
        }));
        capy_game.entity_reserve_manager.add(Box::new(PhysicsEntity {
            name: EntityName::Square,
            position: Vec2::new(225.0, 225.0),
            velocity: Vec2::new(0.0, init_gravity),
            size: Vec2::new(20.0, 20.0),
            should_draw: true,
            texture: capy_game.texture_manager.find(TextureName::Rect),
            collider: Some(AABB{
                min: Vec2::new(300.0, 200.0),
                max: Vec2::new(250.0, 250.0),
                texture: Some(collider_rect_texture.clone()),
                should_draw: draw_colliders,
                inner_collider: None,
                is_colliding: false
            }),
            restitution: 0.5,
            mass: 5.0,
            inv_mass: 1.0 / 5.0
        }));
    }

    capy_game
}

fn update(app: &mut App, capy_game: &mut CapyGame) {
    capy_game.update(app.timer.delta_f32());

    let (mouse_x, mouse_y) = app.mouse.position();

    if app.mouse.was_pressed(MouseButton::Left) {
        let entity: Option<Box<dyn Entity>> = capy_game.entity_reserve_manager.pop();
        match entity {
            Some(mut e) => {
                e.set_position(Vec2{x: mouse_x, y: mouse_y});
                capy_game.entity_active_manager.add(e);
            }
            None => { 
                
            }
        }
    }
    capy_game.mouse_pos_x = mouse_x;
    capy_game.mouse_pos_y = mouse_y;
}

fn draw(gfx: &mut Graphics, capy_game: &mut CapyGame) {
    let mut draw = gfx.create_draw();
    capy_game.draw(gfx, &mut draw);
}
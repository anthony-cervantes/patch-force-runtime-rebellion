use crate::level::{Level, PlatformKind};
use crate::projectile::Weapon;
use crate::sprite;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum PickupKind {
    Weapon(Weapon),
    Health,
    Shield,
}

pub struct Pickup {
    pub pos: Vec2,
    vel: Vec2,
    pub kind: PickupKind,
    pub collected: bool,
    bob: f32,
}

impl Pickup {
    pub fn new(kind: PickupKind, x: f32, y: f32) -> Self {
        Self {
            pos: vec2(x, y),
            vel: vec2(0.0, -40.0),
            kind,
            collected: false,
            bob: x * 0.01,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.pos.x - 13.0, self.pos.y - 13.0, 26.0, 26.0)
    }

    pub fn update(&mut self, dt: f32, level: &Level) {
        self.bob += dt * 4.0;
        self.vel.y += 700.0 * dt;
        self.pos += self.vel * dt;

        let rect = self.rect();
        for platform in &level.platforms {
            match platform.kind {
                PlatformKind::Solid | PlatformKind::Conveyor { .. } => {
                    if rect.overlaps(&platform.rect) && self.vel.y > 0.0 {
                        self.pos.y = platform.rect.y - 13.0;
                        self.vel.y = 0.0;
                    }
                }
            }
        }
    }

    pub fn draw(&self, camera_x: f32, y_offset: f32) {
        let pulse = self.bob.sin() * 2.0;
        let x = self.pos.x - camera_x;
        let y = self.pos.y + y_offset + pulse;
        let (fill, label) = match self.kind {
            PickupKind::Weapon(Weapon::PatchRifle) => (color_u8!(106, 231, 255, 255), "P"),
            PickupKind::Weapon(Weapon::SpreadDiff) => (color_u8!(255, 213, 94, 255), "S"),
            PickupKind::Weapon(Weapon::RefactorBeam) => (color_u8!(176, 255, 155, 255), "R"),
            PickupKind::Weapon(Weapon::HotfixSmg) => (color_u8!(255, 120, 167, 255), "H"),
            PickupKind::Health => (color_u8!(119, 255, 150, 255), "+"),
            PickupKind::Shield => (color_u8!(147, 176, 255, 255), "T"),
        };

        draw_circle_lines(x, y, 20.0 + pulse * 0.5, 2.0, color_u8!(240, 248, 255, 120));
        draw_rectangle(x - 16.0, y - 16.0, 32.0, 32.0, sprite::ink());
        draw_rectangle(x - 12.0, y - 12.0, 24.0, 24.0, color_u8!(20, 25, 42, 255));
        draw_rectangle(x - 9.0, y - 9.0, 18.0, 18.0, fill);
        draw_rectangle(x - 9.0, y - 9.0, 18.0, 3.0, color_u8!(255, 255, 255, 135));
        draw_rectangle(x - 9.0, y + 6.0, 18.0, 3.0, color_u8!(0, 0, 0, 70));
        draw_rectangle_lines(
            x - 16.0,
            y - 16.0,
            32.0,
            32.0,
            3.0,
            color_u8!(240, 248, 255, 255),
        );
        sprite::draw_glint(x - 4.0, y - 4.0, 6.0, color_u8!(255, 255, 255, 210));
        draw_text(label, x - 6.0, y + 7.0, 20.0, color_u8!(7, 10, 21, 255));
    }
}

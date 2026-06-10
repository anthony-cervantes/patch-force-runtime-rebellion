use crate::level::{Level, PlatformKind};
use crate::projectile::Weapon;
use crate::sprite;
use crate::sprite_renderer::SpriteRenderer;
use macroquad::prelude::*;

const PLAYER_W: f32 = 28.0;
const STAND_H: f32 = 48.0;
const CROUCH_H: f32 = 30.0;
const GRAVITY: f32 = 1500.0;
const JUMP_SPEED: f32 = -560.0;
const RUN_SPEED: f32 = 205.0;
const CROUCH_SPEED: f32 = 95.0;

pub struct Player {
    pub pos: Vec2,
    vel: Vec2,
    pub facing: f32,
    pub on_ground: bool,
    pub crouching: bool,
    pub health: i32,
    pub max_health: i32,
    pub lives: i32,
    pub weapon: Weapon,
    pub fire_cooldown: f32,
    pub invuln_timer: f32,
    pub shield_timer: f32,
}

impl Player {
    pub fn new(spawn: Vec2) -> Self {
        Self {
            pos: spawn,
            vel: Vec2::ZERO,
            facing: 1.0,
            on_ground: false,
            crouching: false,
            health: 6,
            max_health: 6,
            lives: 3,
            weapon: Weapon::PatchRifle,
            fire_cooldown: 0.0,
            invuln_timer: 1.0,
            shield_timer: 0.0,
        }
    }

    pub fn reset(&mut self, spawn: Vec2) {
        *self = Self::new(spawn);
    }

    pub fn height(&self) -> f32 {
        if self.crouching && self.on_ground {
            CROUCH_H
        } else {
            STAND_H
        }
    }

    pub fn rect(&self) -> Rect {
        let height = self.height();
        Rect::new(
            self.pos.x - PLAYER_W * 0.5,
            self.pos.y - height,
            PLAYER_W,
            height,
        )
    }

    pub fn center(&self) -> Vec2 {
        let rect = self.rect();
        vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5)
    }

    pub fn is_moving_horizontally(&self) -> bool {
        self.vel.x.abs() > 1.0
    }

    pub fn muzzle_pos(&self, aim: Vec2) -> Vec2 {
        let center = self.center();
        center + aim.normalize_or_zero() * 32.0 + vec2(0.0, if self.crouching { 6.0 } else { -2.0 })
    }

    pub fn update(&mut self, dt: f32, level: &Level) {
        self.fire_cooldown = (self.fire_cooldown - dt).max(0.0);
        self.invuln_timer = (self.invuln_timer - dt).max(0.0);
        self.shield_timer = (self.shield_timer - dt).max(0.0);

        let left = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
        let right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
        let down = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);
        let jump = is_key_pressed(KeyCode::W)
            || is_key_pressed(KeyCode::Up)
            || is_key_pressed(KeyCode::Space);

        let mut axis: f32 = 0.0;
        if left {
            axis -= 1.0;
        }
        if right {
            axis += 1.0;
        }
        if axis != 0.0 {
            self.facing = axis.signum();
        }

        self.crouching = down && self.on_ground;
        self.vel.x = axis
            * if self.crouching {
                CROUCH_SPEED
            } else {
                RUN_SPEED
            };
        if jump && self.on_ground && !self.crouching {
            self.vel.y = JUMP_SPEED;
            self.on_ground = false;
        }

        self.vel.y = (self.vel.y + GRAVITY * dt).min(920.0);

        let previous_rect = self.rect();
        self.pos.x += self.vel.x * dt;
        self.resolve_x(level, previous_rect);
        self.pos.x = self
            .pos
            .x
            .clamp(PLAYER_W * 0.5, level.width - PLAYER_W * 0.5);

        self.on_ground = false;
        let mut conveyor_speed = 0.0;
        let previous_rect = self.rect();
        self.pos.y += self.vel.y * dt;
        self.resolve_y(level, previous_rect, &mut conveyor_speed);
        if self.on_ground && conveyor_speed != 0.0 {
            self.pos.x += conveyor_speed * dt;
        }
    }

    pub fn aim_direction(&self) -> Vec2 {
        let left = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
        let right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
        let up = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
        let down = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);

        let mut x = 0.0;
        if left {
            x -= 1.0;
        }
        if right {
            x += 1.0;
        }

        let mut y = 0.0;
        if up {
            y -= 1.0;
        } else if down && !self.on_ground {
            y += 1.0;
        }

        if x == 0.0 && y == 0.0 {
            x = self.facing;
        }

        vec2(x, y).normalize_or_zero()
    }

    pub fn take_damage(&mut self, amount: i32, respawn: Vec2) -> bool {
        if self.invuln_timer > 0.0 || self.shield_timer > 0.0 {
            return false;
        }

        self.health -= amount;
        self.invuln_timer = 1.2;
        if self.health <= 0 {
            self.lives = (self.lives - 1).max(0);
            if self.lives > 0 {
                self.health = self.max_health;
                self.pos = respawn;
                self.vel = Vec2::ZERO;
                self.invuln_timer = 2.0;
            }
            return true;
        }
        false
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn shield(&mut self, seconds: f32) {
        self.shield_timer = self.shield_timer.max(seconds);
    }

    pub fn draw(&self, renderer: &SpriteRenderer, camera_x: f32, y_offset: f32, aim: Vec2) {
        if self.invuln_timer > 0.0 && (self.invuln_timer * 20.0) as i32 % 2 == 0 {
            return;
        }

        let rect = self.rect();
        let x = rect.x - camera_x;
        let y = rect.y + y_offset;
        let outline = sprite::ink();
        let suit = color_u8!(59, 189, 170, 255);
        let suit_dark = color_u8!(26, 114, 122, 255);
        let trim = color_u8!(255, 213, 94, 255);
        let trim_dark = color_u8!(176, 111, 50, 255);
        let visor = color_u8!(17, 31, 44, 255);
        let glass = color_u8!(124, 237, 255, 255);
        let boot = color_u8!(42, 54, 68, 255);
        let step = if self.on_ground && !self.crouching {
            (get_time() as f32 * 14.0 + self.pos.x * 0.06).sin()
        } else {
            0.0
        };

        if self.shield_timer > 0.0 {
            draw_circle_lines(
                self.center().x - camera_x,
                self.center().y + y_offset,
                34.0 + (self.shield_timer * 8.0).sin() * 2.0,
                3.0,
                color_u8!(147, 176, 255, 210),
            );
            draw_circle_lines(
                self.center().x - camera_x,
                self.center().y + y_offset,
                25.0 + (self.shield_timer * 11.0).cos() * 1.5,
                2.0,
                color_u8!(106, 231, 255, 150),
            );
        }

        let gun_color = match self.weapon {
            Weapon::PatchRifle => color_u8!(106, 231, 255, 255),
            Weapon::SpreadDiff => color_u8!(255, 213, 94, 255),
            Weapon::RefactorBeam => color_u8!(176, 255, 155, 255),
            Weapon::HotfixSmg => color_u8!(255, 120, 167, 255),
        };

        if renderer.draw_player(self, camera_x, y_offset, aim, gun_color) {
            return;
        }

        if self.crouching && self.on_ground {
            sprite::draw_outlined_rect(x + 2.0, y + 16.0, rect.w - 4.0, 12.0, boot);
            sprite::draw_beveled_rect(
                x + 5.0,
                y + 6.0,
                rect.w - 10.0,
                17.0,
                suit,
                color_u8!(114, 236, 217, 255),
                suit_dark,
            );
            sprite::draw_facing_outlined_rect(
                x,
                y,
                rect.w,
                Rect::new(5.0, 0.0, 20.0, 15.0),
                self.facing,
                trim,
            );
            sprite::draw_facing_rect(
                x,
                y,
                rect.w,
                Rect::new(12.0, 5.0, 10.0, 5.0),
                self.facing,
                visor,
            );
            sprite::draw_facing_rect(
                x,
                y,
                rect.w,
                Rect::new(19.0, 5.0, 3.0, 2.0),
                self.facing,
                glass,
            );
        } else {
            let left_leg = step.max(0.0) * 2.0;
            let right_leg = (-step).max(0.0) * 2.0;
            sprite::draw_outlined_rect(x + 5.0, y + rect.h - 14.0 + left_leg, 7.0, 13.0, boot);
            sprite::draw_outlined_rect(
                x + rect.w - 12.0,
                y + rect.h - 14.0 + right_leg,
                7.0,
                13.0,
                boot,
            );
            sprite::draw_beveled_rect(
                x + 5.0,
                y + 18.0,
                rect.w - 10.0,
                rect.h - 29.0,
                suit,
                color_u8!(114, 236, 217, 255),
                suit_dark,
            );
            sprite::draw_facing_rect(
                x,
                y,
                rect.w,
                Rect::new(3.0, 21.0, 6.0, 15.0),
                self.facing,
                color_u8!(36, 137, 138, 255),
            );
            sprite::draw_facing_rect(
                x,
                y,
                rect.w,
                Rect::new(19.0, 21.0, 6.0, 15.0),
                self.facing,
                color_u8!(36, 137, 138, 255),
            );
            sprite::draw_facing_outlined_rect(
                x,
                y,
                rect.w,
                Rect::new(4.0, 2.0, 20.0, 19.0),
                self.facing,
                trim,
            );
            sprite::draw_facing_rect(
                x,
                y,
                rect.w,
                Rect::new(12.0, 7.0, 10.0, 6.0),
                self.facing,
                visor,
            );
            sprite::draw_facing_rect(
                x,
                y,
                rect.w,
                Rect::new(19.0, 8.0, 3.0, 2.0),
                self.facing,
                glass,
            );
            draw_rectangle(x + 11.0, y + 29.0, 6.0, 5.0, trim);
            draw_rectangle(x + 13.0, y + 31.0, 2.0, 2.0, trim_dark);
        }

        let pack_x = sprite::facing_x(x, rect.w, 0.0, 7.0, self.facing);
        draw_rectangle(pack_x, y + 19.0, 7.0, 18.0, outline);
        draw_rectangle(
            pack_x + 2.0,
            y + 21.0,
            5.0,
            14.0,
            color_u8!(55, 71, 84, 255),
        );

        self.draw_gun(
            camera_x,
            y_offset,
            aim.normalize_or_zero(),
            gun_color,
            outline,
        );
    }

    fn draw_gun(&self, camera_x: f32, y_offset: f32, aim: Vec2, gun_color: Color, outline: Color) {
        let dir = if aim.length_squared() > 0.0 {
            aim
        } else {
            vec2(self.facing, 0.0)
        };
        let perp = vec2(-dir.y, dir.x);
        let shoulder =
            self.center() + vec2(self.facing * 5.0, if self.crouching { 8.0 } else { -1.0 });
        let muzzle = self.muzzle_pos(dir);
        let start = shoulder - dir * 7.0;
        let end = muzzle;
        let stock = shoulder - dir * 15.0;
        let grip = shoulder - dir * 2.0 + vec2(0.0, 12.0);
        let sight = shoulder + dir * 10.0 - perp * 6.0;

        draw_line(
            stock.x - camera_x,
            stock.y + y_offset,
            shoulder.x - camera_x,
            shoulder.y + y_offset,
            9.0,
            outline,
        );
        draw_line(
            stock.x - camera_x,
            stock.y + y_offset,
            shoulder.x - camera_x,
            shoulder.y + y_offset,
            5.0,
            color_u8!(42, 54, 68, 255),
        );
        draw_line(
            start.x - camera_x,
            start.y + y_offset,
            end.x - camera_x,
            end.y + y_offset,
            13.0,
            outline,
        );
        draw_line(
            start.x - camera_x,
            start.y + y_offset,
            end.x - camera_x,
            end.y + y_offset,
            8.0,
            color_u8!(35, 44, 58, 255),
        );
        draw_line(
            (start + perp * 2.0).x - camera_x,
            (start + perp * 2.0).y + y_offset,
            (end + perp * 2.0).x - camera_x,
            (end + perp * 2.0).y + y_offset,
            3.5,
            gun_color,
        );
        draw_line(
            sight.x - camera_x,
            sight.y + y_offset,
            (sight + dir * 7.0).x - camera_x,
            (sight + dir * 7.0).y + y_offset,
            3.0,
            outline,
        );
        draw_line(
            shoulder.x - camera_x,
            shoulder.y + y_offset,
            grip.x - camera_x,
            grip.y + y_offset,
            7.0,
            outline,
        );
        draw_line(
            shoulder.x - camera_x,
            shoulder.y + y_offset,
            grip.x - camera_x,
            grip.y + y_offset,
            3.0,
            color_u8!(255, 213, 94, 255),
        );
        draw_circle(end.x - camera_x, end.y + y_offset, 5.0, outline);
        draw_circle(end.x - camera_x, end.y + y_offset, 2.4, gun_color);
    }

    fn resolve_x(&mut self, level: &Level, previous_rect: Rect) {
        for platform in &level.platforms {
            let rect = self.rect();
            if !rect.overlaps(&platform.rect) {
                continue;
            }

            let had_vertical_overlap = previous_rect.y < platform.rect.y + platform.rect.h
                && previous_rect.y + previous_rect.h > platform.rect.y;
            if !had_vertical_overlap {
                continue;
            }

            if self.vel.x > 0.0 && previous_rect.x + previous_rect.w <= platform.rect.x {
                self.pos.x = platform.rect.x - PLAYER_W * 0.5;
                self.vel.x = 0.0;
                break;
            } else if self.vel.x < 0.0 && previous_rect.x >= platform.rect.x + platform.rect.w {
                self.pos.x = platform.rect.x + platform.rect.w + PLAYER_W * 0.5;
                self.vel.x = 0.0;
                break;
            }
        }
    }

    fn resolve_y(&mut self, level: &Level, previous_rect: Rect, conveyor_speed: &mut f32) {
        for platform in &level.platforms {
            let rect = self.rect();
            if !rect.overlaps(&platform.rect) {
                continue;
            }

            let height = self.height();
            if self.vel.y > 0.0 && previous_rect.y + previous_rect.h <= platform.rect.y {
                self.pos.y = platform.rect.y;
                self.vel.y = 0.0;
                self.on_ground = true;
                if let PlatformKind::Conveyor { speed } = platform.kind {
                    *conveyor_speed = speed;
                }
                break;
            } else if self.vel.y < 0.0 && previous_rect.y >= platform.rect.y + platform.rect.h {
                self.pos.y = platform.rect.y + platform.rect.h + height;
                self.vel.y = 0.0;
                break;
            }
        }
    }
}

use crate::enemy::{Enemy, EnemyKind};
use crate::particle::Particle;
use crate::projectile::Projectile;
use crate::sprite;
use crate::sprite_renderer::SpriteRenderer;
use macroquad::prelude::*;

pub struct Boss {
    pub pos: Vec2,
    size: Vec2,
    pub health: i32,
    pub max_health: i32,
    pub active: bool,
    pub defeated: bool,
    attack_timer: f32,
    spawn_timer: f32,
    slam_timer: f32,
    flash_timer: f32,
}

impl Boss {
    pub fn new() -> Self {
        Self {
            pos: vec2(5600.0, 286.0),
            size: vec2(190.0, 194.0),
            health: 120,
            max_health: 120,
            active: false,
            defeated: false,
            attack_timer: 1.0,
            spawn_timer: 3.0,
            slam_timer: 4.4,
            flash_timer: 0.0,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y)
    }

    pub fn is_flashing(&self) -> bool {
        self.flash_timer > 0.0
    }

    pub fn center(&self) -> Vec2 {
        self.pos + self.size * 0.5
    }

    pub fn update(
        &mut self,
        dt: f32,
        player_pos: Vec2,
        projectiles: &mut Vec<Projectile>,
        spawned: &mut Vec<Enemy>,
        particles: &mut Vec<Particle>,
        shake: &mut f32,
    ) {
        if !self.active || self.defeated {
            return;
        }

        self.flash_timer = (self.flash_timer - dt).max(0.0);
        let speed = if self.health <= self.max_health / 3 {
            0.55
        } else if self.health <= self.max_health * 2 / 3 {
            0.75
        } else {
            1.0
        };

        self.attack_timer -= dt;
        self.spawn_timer -= dt;
        self.slam_timer -= dt;

        if self.attack_timer <= 0.0 {
            self.attack_timer = 1.55 * speed;
            let origin = self.pos + vec2(36.0, 86.0);
            let dir = (player_pos - origin).normalize_or_zero();
            for angle in [-0.18_f32, 0.0, 0.18] {
                projectiles.push(Projectile::conflict(
                    origin,
                    rotate(dir, angle),
                    250.0 / speed,
                ));
            }
        }

        if self.spawn_timer <= 0.0 {
            self.spawn_timer = 4.8 * speed;
            spawned.push(Enemy::new(
                EnemyKind::BugCrawler,
                self.pos.x - 120.0,
                456.0,
                None,
            ));
            particles.extend(Particle::burst(
                self.pos + vec2(6.0, 166.0),
                color_u8!(152, 231, 89, 255),
                10,
                130.0,
            ));
        }

        if self.slam_timer <= 0.0 {
            self.slam_timer = 5.0 * speed;
            projectiles.push(Projectile::shockwave(
                self.pos + vec2(12.0, 469.0 - self.pos.y),
                -1.0,
            ));
            projectiles.push(Projectile::shockwave(
                self.pos + vec2(120.0, 469.0 - self.pos.y),
                1.0,
            ));
            particles.extend(Particle::burst(
                self.pos + vec2(80.0, 184.0),
                color_u8!(255, 180, 82, 255),
                18,
                210.0,
            ));
            *shake = shake.max(0.38);
        }
    }

    pub fn hit(&mut self, damage: i32, particles: &mut Vec<Particle>) -> bool {
        if !self.active || self.defeated {
            return false;
        }
        self.health -= damage;
        self.flash_timer = 0.12;
        particles.extend(Particle::burst(
            self.center(),
            color_u8!(214, 126, 255, 255),
            5,
            105.0,
        ));
        if self.health <= 0 {
            self.health = 0;
            self.defeated = true;
            particles.extend(Particle::burst(
                self.center(),
                color_u8!(255, 213, 94, 255),
                42,
                300.0,
            ));
            return true;
        }
        false
    }

    pub fn draw(&self, renderer: &SpriteRenderer, camera_x: f32, y_offset: f32) {
        if !self.active || self.defeated {
            return;
        }

        if renderer.draw_boss(self, camera_x, y_offset) {
            return;
        }

        let x = self.pos.x - camera_x;
        let y = self.pos.y + y_offset;
        let outline = sprite::ink();
        let armor = if self.flash_timer > 0.0 {
            WHITE
        } else {
            color_u8!(98, 81, 131, 255)
        };
        let armor_dark = color_u8!(47, 35, 68, 255);
        let armor_light = color_u8!(143, 118, 176, 255);
        let accent = color_u8!(214, 126, 255, 255);
        let hot = color_u8!(255, 103, 97, 255);
        let phase = self.health as f32 / self.max_health as f32;
        let angry = phase <= 0.34;
        let time = get_time() as f32;
        let pulse = (time * if angry { 16.0 } else { 8.0 }).sin();
        let glow = (time * 3.4).sin() * 0.5 + 0.5;

        draw_circle_lines(
            x + 89.0,
            y + 104.0,
            96.0 + glow * 6.0,
            4.0,
            color_u8!(214, 126, 255, 80),
        );
        draw_line(
            x + 14.0,
            y + 34.0,
            x - 34.0,
            y + 8.0 + pulse * 3.0,
            5.0,
            color_u8!(34, 24, 45, 255),
        );
        draw_line(
            x + 164.0,
            y + 34.0,
            x + 220.0,
            y + 10.0 - pulse * 3.0,
            5.0,
            color_u8!(34, 24, 45, 255),
        );
        draw_line(
            x + 24.0,
            y + 58.0,
            x + 0.0,
            y + 147.0,
            4.0,
            color_u8!(214, 126, 255, 90),
        );
        draw_line(
            x + 154.0,
            y + 58.0,
            x + 184.0,
            y + 147.0,
            4.0,
            color_u8!(214, 126, 255, 90),
        );

        draw_rectangle(
            x - 8.0,
            y - 8.0,
            self.size.x + 16.0,
            self.size.y + 16.0,
            outline,
        );
        sprite::draw_beveled_rect(
            x - 22.0,
            y + 48.0,
            62.0,
            62.0,
            armor,
            armor_light,
            armor_dark,
        );
        sprite::draw_beveled_rect(
            x + 150.0,
            y + 48.0,
            62.0,
            62.0,
            armor,
            armor_light,
            armor_dark,
        );
        draw_rectangle(x - 12.0, y + 64.0, 42.0, 12.0, armor_dark);
        draw_rectangle(x + 160.0, y + 64.0, 42.0, 12.0, armor_dark);
        draw_rectangle(x - 6.0, y + 86.0, 30.0, 10.0, hot);
        draw_rectangle(x + 166.0, y + 86.0, 30.0, 10.0, hot);
        draw_rectangle(x + 18.0, y + 42.0, 154.0, 98.0, armor_dark);
        sprite::draw_beveled_rect(x + 36.0, y, 104.0, 72.0, armor, armor_light, armor_dark);
        draw_rectangle(x + 82.0, y - 20.0, 14.0, 22.0, outline);
        draw_rectangle(x + 85.0, y - 24.0, 8.0, 18.0, hot);
        draw_line(x + 55.0, y + 3.0, x + 34.0, y - 22.0, 4.0, outline);
        draw_line(x + 124.0, y + 3.0, x + 146.0, y - 22.0, 4.0, outline);
        draw_circle(x + 34.0, y - 22.0, 5.0, accent);
        draw_circle(x + 146.0, y - 22.0, 5.0, accent);
        draw_rectangle(x + 48.0, y + 14.0, 80.0, 30.0, color_u8!(32, 24, 45, 255));
        draw_rectangle(x + 52.0, y + 18.0, 72.0 * phase.max(0.08), 4.0, accent);
        draw_text("<<<<<<<", x + 52.0, y + 34.0, 18.0, hot);
        draw_rectangle(x + 57.0, y + 45.0, 12.0, 10.0, hot);
        draw_rectangle(x + 108.0, y + 45.0, 12.0, 10.0, hot);
        draw_rectangle(x + 72.0, y + 48.0, 34.0, 9.0, color_u8!(28, 22, 35, 255));
        draw_rectangle(x + 76.0, y + 51.0, 26.0, 3.0, color_u8!(255, 213, 94, 180));
        sprite::draw_beveled_rect(
            x + 18.0,
            y + 72.0,
            142.0,
            80.0,
            armor,
            armor_light,
            armor_dark,
        );
        draw_rectangle(x + 34.0, y + 88.0, 110.0, 24.0, color_u8!(34, 24, 45, 255));
        draw_circle(x + 89.0, y + 100.0, 20.0, outline);
        draw_circle(
            x + 89.0,
            y + 100.0,
            15.0 + glow * 2.0,
            color_u8!(214, 126, 255, 180),
        );
        draw_circle(x + 89.0, y + 100.0, 7.0, color_u8!(255, 238, 180, 220));
        draw_rectangle(x + 38.0, y + 92.0, 102.0, 5.0, accent);
        draw_rectangle(x + 38.0, y + 104.0, 72.0 + pulse * 8.0, 5.0, hot);
        draw_rectangle(x + 44.0, y + 121.0, 18.0, 14.0, color_u8!(37, 31, 45, 255));
        draw_rectangle(x + 76.0, y + 121.0, 18.0, 14.0, color_u8!(37, 31, 45, 255));
        draw_rectangle(x + 108.0, y + 121.0, 18.0, 14.0, color_u8!(37, 31, 45, 255));
        draw_line(x + 18.0, y + 78.0, x + 4.0, y + 46.0, 5.0, outline);
        draw_line(x + 160.0, y + 78.0, x + 177.0, y + 46.0, 5.0, outline);
        draw_piston(
            x + 24.0,
            y + 122.0,
            x + 54.0,
            y + 155.0,
            armor_light,
            outline,
        );
        draw_piston(
            x + 156.0,
            y + 122.0,
            x + 126.0,
            y + 155.0,
            armor_light,
            outline,
        );
        sprite::draw_beveled_rect(x, y + 108.0, 38.0, 24.0, armor, armor_light, armor_dark);
        sprite::draw_beveled_rect(
            x + 152.0,
            y + 108.0,
            38.0,
            24.0,
            armor,
            armor_light,
            armor_dark,
        );
        draw_rectangle(x - 36.0, y + 112.0, 38.0, 12.0, outline);
        draw_rectangle(x + 188.0, y + 112.0, 38.0, 12.0, outline);
        draw_rectangle(x - 35.0, y + 116.0, 40.0, 4.0, hot);
        draw_rectangle(x + 187.0, y + 116.0, 40.0, 4.0, hot);
        draw_triangle(
            vec2(x - 39.0, y + 109.0),
            vec2(x - 58.0, y + 119.0),
            vec2(x - 39.0, y + 130.0),
            outline,
        );
        draw_triangle(
            vec2(x + 229.0, y + 109.0),
            vec2(x + 252.0, y + 119.0),
            vec2(x + 229.0, y + 130.0),
            outline,
        );
        draw_circle(x - 58.0, y + 119.0, 7.0, hot);
        draw_circle(x + 252.0, y + 119.0, 7.0, hot);
        sprite::draw_beveled_rect(
            x + 30.0,
            y + 152.0,
            34.0,
            42.0,
            armor,
            armor_light,
            armor_dark,
        );
        sprite::draw_beveled_rect(
            x + 116.0,
            y + 152.0,
            34.0,
            42.0,
            armor,
            armor_light,
            armor_dark,
        );
        draw_rectangle(x + 36.0, y + 164.0, 22.0, 6.0, armor_dark);
        draw_rectangle(x + 122.0, y + 164.0, 22.0, 6.0, armor_dark);
        draw_piston(
            x + 45.0,
            y + 176.0,
            x + 37.0,
            y + 193.0,
            armor_light,
            outline,
        );
        draw_piston(
            x + 131.0,
            y + 176.0,
            x + 123.0,
            y + 193.0,
            armor_light,
            outline,
        );
        draw_rectangle(x + 25.0, y + 188.0, 52.0, 12.0, outline);
        draw_rectangle(x + 105.0, y + 188.0, 52.0, 12.0, outline);
        draw_rectangle(x + 35.0, y + 186.0, 32.0, 5.0, hot);
        draw_rectangle(x + 115.0, y + 186.0, 32.0, 5.0, hot);

        if angry {
            draw_circle_lines(
                x + 89.0,
                y + 100.0,
                66.0 + pulse * 2.0,
                3.0,
                color_u8!(255, 103, 97, 155),
            );
        }
    }
}

fn draw_piston(x1: f32, y1: f32, x2: f32, y2: f32, color: Color, outline: Color) {
    draw_line(x1, y1, x2, y2, 7.0, outline);
    draw_line(x1, y1, x2, y2, 3.0, color);
    draw_circle(x1, y1, 5.0, outline);
    draw_circle(x2, y2, 5.0, outline);
    draw_circle(x1, y1, 2.0, color);
    draw_circle(x2, y2, 2.0, color);
}

fn rotate(v: Vec2, radians: f32) -> Vec2 {
    let (s, c) = radians.sin_cos();
    vec2(v.x * c - v.y * s, v.x * s + v.y * c)
}

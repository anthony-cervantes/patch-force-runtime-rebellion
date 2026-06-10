use crate::level::{Level, PlatformKind};
use crate::pickup::PickupKind;
use crate::projectile::Projectile;
use crate::sprite;
use crate::sprite_renderer::SpriteRenderer;
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnemyKind {
    BugCrawler,
    ExceptionBat,
    BuildTurret,
    TestBot,
}

pub struct Enemy {
    pub kind: EnemyKind,
    pub pos: Vec2,
    vel: Vec2,
    spawn: Vec2,
    dir: f32,
    health: i32,
    pub alive: bool,
    pub drop: Option<PickupKind>,
    shoot_timer: f32,
    ai_timer: f32,
    flash_timer: f32,
}

impl Enemy {
    pub fn new(kind: EnemyKind, x: f32, y: f32, drop: Option<PickupKind>) -> Self {
        let health = match kind {
            EnemyKind::BugCrawler => 3,
            EnemyKind::ExceptionBat => 2,
            EnemyKind::BuildTurret => 6,
            EnemyKind::TestBot => 4,
        };
        Self {
            kind,
            pos: vec2(x, y),
            vel: Vec2::ZERO,
            spawn: vec2(x, y),
            dir: -1.0,
            health,
            alive: true,
            drop,
            shoot_timer: 0.8,
            ai_timer: x * 0.01,
            flash_timer: 0.0,
        }
    }

    pub fn rect(&self) -> Rect {
        let size = self.size();
        Rect::new(self.pos.x, self.pos.y, size.x, size.y)
    }

    pub fn center(&self) -> Vec2 {
        let rect = self.rect();
        vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5)
    }

    pub fn facing(&self) -> f32 {
        self.dir
    }

    pub fn is_flashing(&self) -> bool {
        self.flash_timer > 0.0
    }

    pub fn hit(&mut self, damage: i32) -> bool {
        self.health -= damage;
        self.flash_timer = 0.1;
        if self.health <= 0 {
            self.alive = false;
            return true;
        }
        false
    }

    pub fn update(
        &mut self,
        dt: f32,
        player_pos: Vec2,
        level: &Level,
        shots: &mut Vec<Projectile>,
    ) {
        if !self.alive {
            return;
        }

        self.ai_timer += dt;
        self.flash_timer = (self.flash_timer - dt).max(0.0);

        match self.kind {
            EnemyKind::BugCrawler => self.update_ground(dt, level, 54.0, false, player_pos),
            EnemyKind::TestBot => self.update_ground(dt, level, 100.0, true, player_pos),
            EnemyKind::ExceptionBat => self.update_bat(dt, player_pos),
            EnemyKind::BuildTurret => self.update_turret(dt, player_pos, shots),
        }
    }

    pub fn draw(&self, renderer: &SpriteRenderer, camera_x: f32, y_offset: f32) {
        if !self.alive {
            return;
        }

        if renderer.draw_enemy(self, camera_x, y_offset) {
            return;
        }

        let rect = self.rect();
        let x = rect.x - camera_x;
        let y = rect.y + y_offset;
        let flash = self.flash_timer > 0.0;
        let outline = sprite::ink();
        let fill = if flash {
            WHITE
        } else {
            match self.kind {
                EnemyKind::BugCrawler => color_u8!(152, 231, 89, 255),
                EnemyKind::ExceptionBat => color_u8!(201, 113, 255, 255),
                EnemyKind::BuildTurret => color_u8!(235, 113, 80, 255),
                EnemyKind::TestBot => color_u8!(255, 200, 76, 255),
            }
        };

        match self.kind {
            EnemyKind::BugCrawler => {
                let shell_dark = color_u8!(55, 104, 54, 255);
                let leg_phase = (get_time() as f32 * 12.0 + self.pos.x * 0.05).sin();
                for i in 0..3 {
                    let lx = x + 7.0 + i as f32 * 10.0;
                    let lift = if (i % 2 == 0) == (leg_phase > 0.0) {
                        -2.0
                    } else {
                        2.0
                    };
                    draw_line(lx, y + 16.0, lx - 7.0, y + 23.0 + lift, 3.0, outline);
                    draw_line(lx + 7.0, y + 16.0, lx + 14.0, y + 23.0 - lift, 3.0, outline);
                }
                sprite::draw_beveled_rect(
                    x + 2.0,
                    y + 7.0,
                    rect.w - 4.0,
                    rect.h - 9.0,
                    fill,
                    color_u8!(198, 255, 126, 255),
                    shell_dark,
                );
                sprite::draw_outlined_rect(x + 9.0, y + 1.0, rect.w - 18.0, 12.0, fill);
                draw_rectangle(x + 12.0, y + 10.0, 3.0, 8.0, shell_dark);
                draw_rectangle(x + 21.0, y + 10.0, 3.0, 8.0, shell_dark);
                draw_rectangle(x + 7.0, y + 8.0, 5.0, 4.0, color_u8!(255, 255, 190, 255));
                draw_rectangle(
                    x + rect.w - 12.0,
                    y + 8.0,
                    5.0,
                    4.0,
                    color_u8!(255, 255, 190, 255),
                );
                draw_line(x + 8.0, y + 4.0, x + 0.0, y - 2.0, 2.0, outline);
                draw_line(x + rect.w - 8.0, y + 4.0, x + rect.w, y - 2.0, 2.0, outline);
            }
            EnemyKind::ExceptionBat => {
                let flap = (get_time() as f32 * 15.0 + self.spawn.x * 0.03).sin() * 7.0;
                draw_triangle(
                    vec2(x + 17.0, y + 9.0),
                    vec2(x - 18.0, y + 22.0 - flap),
                    vec2(x + 13.0, y + 27.0),
                    outline,
                );
                draw_triangle(
                    vec2(x + 21.0, y + 9.0),
                    vec2(x + rect.w + 18.0, y + 22.0 - flap),
                    vec2(x + 25.0, y + 27.0),
                    outline,
                );
                draw_triangle(
                    vec2(x + 17.0, y + 12.0),
                    vec2(x - 9.0, y + 22.0 - flap),
                    vec2(x + 14.0, y + 23.0),
                    fill,
                );
                draw_triangle(
                    vec2(x + 21.0, y + 12.0),
                    vec2(x + rect.w + 9.0, y + 22.0 - flap),
                    vec2(x + 24.0, y + 23.0),
                    fill,
                );
                sprite::draw_beveled_rect(
                    x + 9.0,
                    y + 6.0,
                    rect.w - 18.0,
                    rect.h - 10.0,
                    fill,
                    color_u8!(237, 166, 255, 255),
                    color_u8!(92, 40, 133, 255),
                );
                draw_rectangle(x + 15.0, y + 16.0, 4.0, 4.0, color_u8!(255, 255, 190, 255));
                draw_rectangle(x + 22.0, y + 16.0, 4.0, 4.0, color_u8!(255, 255, 190, 255));
                draw_rectangle(x + 18.0, y + 22.0, 4.0, 4.0, outline);
            }
            EnemyKind::BuildTurret => {
                sprite::draw_beveled_rect(
                    x + 4.0,
                    y + 16.0,
                    rect.w - 8.0,
                    rect.h - 16.0,
                    color_u8!(85, 67, 69, 255),
                    color_u8!(129, 110, 110, 255),
                    color_u8!(42, 31, 39, 255),
                );
                sprite::draw_outlined_rect(x + 8.0, y + 2.0, rect.w - 16.0, 22.0, fill);
                draw_rectangle(
                    x + 13.0,
                    y + 8.0,
                    rect.w - 26.0,
                    8.0,
                    color_u8!(38, 25, 30, 255),
                );
                draw_rectangle(
                    x + 15.0,
                    y + 10.0,
                    (rect.w - 30.0) * (self.health.max(1) as f32 / 6.0),
                    4.0,
                    color_u8!(255, 215, 110, 255),
                );
                let barrel_x = if self.dir >= 0.0 {
                    x + rect.w - 1.0
                } else {
                    x - 19.0
                };
                draw_rectangle(barrel_x, y + 8.0, 20.0, 8.0, outline);
                draw_rectangle(barrel_x, y + 10.0, 20.0, 4.0, fill);
                draw_circle(x + rect.w * 0.5, y + 30.0, 6.0, outline);
                draw_circle(x + rect.w * 0.5, y + 30.0, 3.0, color_u8!(255, 69, 82, 255));
            }
            EnemyKind::TestBot => {
                let run = (get_time() as f32 * 18.0 + self.pos.x * 0.08).sin();
                sprite::draw_outlined_rect(
                    x + 5.0,
                    y + rect.h - 7.0 + run.max(0.0),
                    8.0,
                    9.0,
                    outline,
                );
                sprite::draw_outlined_rect(
                    x + rect.w - 13.0,
                    y + rect.h - 7.0 + (-run).max(0.0),
                    8.0,
                    9.0,
                    outline,
                );
                sprite::draw_beveled_rect(
                    x + 2.0,
                    y + 5.0,
                    rect.w - 4.0,
                    rect.h - 10.0,
                    fill,
                    color_u8!(255, 235, 131, 255),
                    color_u8!(172, 113, 38, 255),
                );
                draw_rectangle(
                    x + 6.0,
                    y + 10.0,
                    rect.w - 12.0,
                    9.0,
                    color_u8!(24, 31, 40, 255),
                );
                draw_rectangle(x + 9.0, y + 13.0, 4.0, 3.0, color_u8!(255, 89, 94, 255));
                draw_rectangle(x + 19.0, y + 13.0, 4.0, 3.0, color_u8!(255, 89, 94, 255));
                draw_line(x + 3.0, y + 24.0, x - 5.0, y + 30.0, 3.0, outline);
                draw_line(
                    x + rect.w - 3.0,
                    y + 24.0,
                    x + rect.w + 5.0,
                    y + 30.0,
                    3.0,
                    outline,
                );
            }
        }
    }

    fn size(&self) -> Vec2 {
        match self.kind {
            EnemyKind::BugCrawler => vec2(36.0, 24.0),
            EnemyKind::ExceptionBat => vec2(38.0, 30.0),
            EnemyKind::BuildTurret => vec2(42.0, 44.0),
            EnemyKind::TestBot => vec2(32.0, 34.0),
        }
    }

    fn update_ground(
        &mut self,
        dt: f32,
        level: &Level,
        speed: f32,
        chases_player: bool,
        player_pos: Vec2,
    ) {
        if chases_player && (player_pos.x - self.pos.x).abs() < 580.0 {
            self.dir = (player_pos.x - self.pos.x).signum();
            if self.dir == 0.0 {
                self.dir = 1.0;
            }
        } else if (self.pos.x - self.spawn.x).abs() > 145.0 {
            self.dir *= -1.0;
        }

        self.vel.x = self.dir * speed;
        self.vel.y = (self.vel.y + 1200.0 * dt).min(760.0);
        self.pos.x += self.vel.x * dt;
        if self.collide_x(level) {
            self.dir *= -1.0;
        }
        self.pos.y += self.vel.y * dt;
        self.collide_y(level);
    }

    fn update_bat(&mut self, dt: f32, player_pos: Vec2) {
        let home = self.spawn
            + vec2(
                (self.ai_timer * 1.7).sin() * 95.0,
                (self.ai_timer * 4.0).sin() * 34.0,
            );
        let to_player = player_pos - self.center();
        if to_player.length() < 360.0 {
            self.pos += to_player.normalize_or_zero() * 128.0 * dt;
            self.pos.y += (self.ai_timer * 7.0).sin() * 24.0 * dt;
        } else {
            self.pos = self.pos.lerp(home, (dt * 1.8).min(1.0));
        }
    }

    fn update_turret(&mut self, dt: f32, player_pos: Vec2, shots: &mut Vec<Projectile>) {
        self.dir = (player_pos.x - self.center().x).signum();
        if self.dir == 0.0 {
            self.dir = -1.0;
        }
        self.shoot_timer -= dt;
        let to_player = player_pos - self.center();
        if to_player.length() < 720.0 && self.shoot_timer <= 0.0 {
            self.shoot_timer = 1.65;
            shots.push(Projectile::enemy(self.center(), to_player));
        }
    }

    fn collide_x(&mut self, level: &Level) -> bool {
        let rect = self.rect();
        for platform in &level.platforms {
            if rect.overlaps(&platform.rect) {
                let size = self.size();
                if self.vel.x > 0.0 {
                    self.pos.x = platform.rect.x - size.x;
                } else if self.vel.x < 0.0 {
                    self.pos.x = platform.rect.x + platform.rect.w;
                }
                return true;
            }
        }
        false
    }

    fn collide_y(&mut self, level: &Level) {
        let rect = self.rect();
        for platform in &level.platforms {
            match platform.kind {
                PlatformKind::Solid | PlatformKind::Conveyor { .. } => {
                    if rect.overlaps(&platform.rect) && self.vel.y > 0.0 {
                        let size = self.size();
                        self.pos.y = platform.rect.y - size.y;
                        self.vel.y = 0.0;
                    }
                }
            }
        }
    }
}

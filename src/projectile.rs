use crate::sprite;
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Weapon {
    PatchRifle,
    SpreadDiff,
    RefactorBeam,
    HotfixSmg,
}

impl Weapon {
    pub fn name(self) -> &'static str {
        match self {
            Weapon::PatchRifle => "Patch Rifle",
            Weapon::SpreadDiff => "Spread Diff",
            Weapon::RefactorBeam => "Refactor Beam",
            Weapon::HotfixSmg => "Hotfix SMG",
        }
    }

    pub fn fire_delay(self) -> f32 {
        match self {
            Weapon::PatchRifle => 0.22,
            Weapon::SpreadDiff => 0.34,
            Weapon::RefactorBeam => 0.48,
            Weapon::HotfixSmg => 0.09,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectileOwner {
    Player,
    Enemy,
    Boss,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectileKind {
    Patch,
    Spread,
    Beam,
    Hotfix,
    EnemyShot,
    Conflict,
    Shockwave,
}

pub struct Projectile {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub damage: i32,
    pub owner: ProjectileOwner,
    pub kind: ProjectileKind,
    pub ttl: f32,
    pub piercing: bool,
    pub alive: bool,
    color: Color,
}

impl Projectile {
    pub fn player(origin: Vec2, dir: Vec2, weapon: Weapon) -> Vec<Self> {
        let dir = dir.normalize_or_zero();
        match weapon {
            Weapon::PatchRifle => vec![Self::new(
                origin,
                dir * 620.0,
                4.0,
                2,
                ProjectileOwner::Player,
                ProjectileKind::Patch,
                1.4,
                false,
                color_u8!(106, 231, 255, 255),
            )],
            Weapon::SpreadDiff => [-0.24_f32, 0.0, 0.24]
                .iter()
                .map(|angle| {
                    Self::new(
                        origin,
                        rotate(dir, *angle) * 560.0,
                        3.7,
                        1,
                        ProjectileOwner::Player,
                        ProjectileKind::Spread,
                        1.1,
                        false,
                        color_u8!(255, 213, 94, 255),
                    )
                })
                .collect(),
            Weapon::RefactorBeam => vec![Self::new(
                origin,
                dir * 700.0,
                6.0,
                4,
                ProjectileOwner::Player,
                ProjectileKind::Beam,
                0.9,
                true,
                color_u8!(176, 255, 155, 255),
            )],
            Weapon::HotfixSmg => vec![Self::new(
                origin,
                dir * 760.0,
                3.0,
                1,
                ProjectileOwner::Player,
                ProjectileKind::Hotfix,
                0.8,
                false,
                color_u8!(255, 120, 167, 255),
            )],
        }
    }

    pub fn enemy(origin: Vec2, dir: Vec2) -> Self {
        Self::new(
            origin,
            dir.normalize_or_zero() * 250.0,
            5.0,
            1,
            ProjectileOwner::Enemy,
            ProjectileKind::EnemyShot,
            3.0,
            false,
            color_u8!(255, 92, 71, 255),
        )
    }

    pub fn conflict(origin: Vec2, dir: Vec2, speed: f32) -> Self {
        Self::new(
            origin,
            dir.normalize_or_zero() * speed,
            8.0,
            1,
            ProjectileOwner::Boss,
            ProjectileKind::Conflict,
            3.4,
            false,
            color_u8!(218, 117, 255, 255),
        )
    }

    pub fn shockwave(origin: Vec2, direction: f32) -> Self {
        Self::new(
            origin,
            vec2(direction.signum() * 310.0, 0.0),
            13.0,
            1,
            ProjectileOwner::Boss,
            ProjectileKind::Shockwave,
            2.6,
            false,
            color_u8!(255, 180, 82, 255),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        pos: Vec2,
        vel: Vec2,
        radius: f32,
        damage: i32,
        owner: ProjectileOwner,
        kind: ProjectileKind,
        ttl: f32,
        piercing: bool,
        color: Color,
    ) -> Self {
        Self {
            pos,
            vel,
            radius,
            damage,
            owner,
            kind,
            ttl,
            piercing,
            alive: true,
            color,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.ttl -= dt;
        if self.ttl <= 0.0 {
            self.alive = false;
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(
            self.pos.x - self.radius,
            self.pos.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }

    pub fn draw(&self, camera_x: f32, y_offset: f32) {
        let x = self.pos.x - camera_x;
        let y = self.pos.y + y_offset;
        match self.kind {
            ProjectileKind::Beam => {
                let tail = self.vel.normalize_or_zero() * -22.0;
                let head = self.vel.normalize_or_zero() * 8.0;
                draw_line(
                    x + tail.x,
                    y + tail.y,
                    x,
                    y,
                    6.0,
                    color_u8!(22, 42, 35, 255),
                );
                draw_line(x + tail.x, y + tail.y, x, y, 3.0, self.color);
                draw_line(
                    x,
                    y,
                    x + head.x,
                    y + head.y,
                    2.0,
                    color_u8!(240, 255, 221, 255),
                );
                draw_circle(x, y, self.radius, color_u8!(22, 42, 35, 255));
                draw_circle(x, y, self.radius - 2.0, self.color);
            }
            ProjectileKind::Conflict => {
                draw_rectangle(x - 11.0, y - 8.0, 22.0, 16.0, color_u8!(35, 16, 47, 255));
                draw_rectangle_lines(x - 11.0, y - 8.0, 22.0, 16.0, 2.0, self.color);
                draw_text("<<<", x - 10.0, y + 5.0, 13.0, self.color);
            }
            ProjectileKind::Shockwave => {
                draw_rectangle(x - 16.0, y - 9.0, 32.0, 18.0, color_u8!(53, 34, 15, 255));
                draw_triangle(
                    vec2(x - 18.0, y + 9.0),
                    vec2(x, y - 16.0),
                    vec2(x + 18.0, y + 9.0),
                    self.color,
                );
            }
            ProjectileKind::Patch | ProjectileKind::Spread | ProjectileKind::Hotfix => {
                let dir = self.vel.normalize_or_zero();
                let tail = dir * -11.0;
                draw_line(
                    x + tail.x,
                    y + tail.y,
                    x,
                    y,
                    self.radius + 3.0,
                    sprite::deep_shadow(),
                );
                draw_line(x + tail.x, y + tail.y, x, y, self.radius, self.color);
                draw_circle(x, y, self.radius + 2.0, sprite::deep_shadow());
                draw_circle(x, y, self.radius, self.color);
                draw_circle(
                    x + dir.x * 2.0,
                    y + dir.y * 2.0,
                    1.5,
                    color_u8!(255, 255, 255, 210),
                );
            }
            _ => {
                draw_circle(x, y, self.radius + 2.0, color_u8!(11, 16, 32, 255));
                draw_circle(x, y, self.radius, self.color);
            }
        }
    }
}

fn rotate(v: Vec2, radians: f32) -> Vec2 {
    let (s, c) = radians.sin_cos();
    vec2(v.x * c - v.y * s, v.x * s + v.y * c)
}

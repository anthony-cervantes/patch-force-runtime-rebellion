use crate::sprite;
use crate::sprite_renderer::SpriteRenderer;
use macroquad::prelude::*;

pub const SCREEN_W: f32 = 960.0;
pub const SCREEN_H: f32 = 540.0;
pub const GROUND_Y: f32 = 480.0;

#[derive(Clone, Copy)]
pub enum PlatformKind {
    Solid,
    Conveyor { speed: f32 },
}

#[derive(Clone, Copy)]
pub struct Platform {
    pub rect: Rect,
    pub kind: PlatformKind,
}

#[derive(Clone, Copy)]
pub struct Hazard {
    pub rect: Rect,
    pub phase: f32,
}

pub struct Level {
    pub width: f32,
    pub boss_start: f32,
    pub platforms: Vec<Platform>,
    pub hazards: Vec<Hazard>,
}

impl Level {
    pub fn new() -> Self {
        let mut platforms = vec![
            solid(0.0, GROUND_Y, 2140.0, 80.0),
            solid(2140.0, GROUND_Y, 2180.0, 80.0),
            solid(4320.0, GROUND_Y, 2120.0, 80.0),
            solid(330.0, 390.0, 210.0, 28.0),
            solid(690.0, 330.0, 210.0, 28.0),
            solid(1040.0, 385.0, 250.0, 28.0),
            solid(1430.0, 315.0, 220.0, 28.0),
            solid(1740.0, 405.0, 250.0, 28.0),
            conveyor(2240.0, 432.0, 560.0, 30.0, 70.0),
            conveyor(3030.0, 432.0, 540.0, 30.0, -85.0),
            solid(2490.0, 338.0, 230.0, 28.0),
            solid(2870.0, 292.0, 230.0, 28.0),
            conveyor(3590.0, 392.0, 360.0, 30.0, 105.0),
            solid(3920.0, 318.0, 240.0, 28.0),
            solid(4600.0, 390.0, 230.0, 28.0),
            solid(5100.0, 360.0, 200.0, 28.0),
            solid(5480.0, 408.0, 210.0, 28.0),
        ];

        platforms.push(solid(6175.0, 260.0, 34.0, 220.0));

        let hazards = vec![
            Hazard {
                rect: Rect::new(2600.0, 348.0, 24.0, 132.0),
                phase: 0.0,
            },
            Hazard {
                rect: Rect::new(3340.0, 322.0, 24.0, 158.0),
                phase: 1.4,
            },
            Hazard {
                rect: Rect::new(3820.0, 220.0, 28.0, 172.0),
                phase: 2.1,
            },
        ];

        Self {
            width: 6440.0,
            boss_start: 4964.0,
            platforms,
            hazards,
        }
    }

    pub fn section_name(&self, x: f32) -> &'static str {
        if x < 2140.0 {
            "Legacy Jungle"
        } else if x < self.boss_start {
            "CI/CD Factory"
        } else {
            "Production Core"
        }
    }

    pub fn hazard_active(&self, index: usize, time: f32) -> bool {
        self.hazards
            .get(index)
            .map(|hazard| (time * 2.7 + hazard.phase).sin() > -0.25)
            .unwrap_or(false)
    }

    pub fn draw(&self, renderer: &SpriteRenderer, camera_x: f32, y_offset: f32, time: f32) {
        clear_background(color_u8!(11, 16, 32, 255));
        self.draw_background(renderer, camera_x, y_offset, time);

        for platform in &self.platforms {
            if !visible(platform.rect, camera_x) {
                continue;
            }
            let x = platform.rect.x - camera_x;
            let y = platform.rect.y + y_offset;
            let (fill, trim) = match platform.kind {
                PlatformKind::Solid => {
                    if platform.rect.x < 2140.0 {
                        (color_u8!(47, 88, 60, 255), color_u8!(142, 219, 122, 255))
                    } else if platform.rect.x < self.boss_start {
                        (color_u8!(71, 74, 91, 255), color_u8!(255, 190, 84, 255))
                    } else {
                        (color_u8!(65, 48, 87, 255), color_u8!(214, 126, 255, 255))
                    }
                }
                PlatformKind::Conveyor { speed } => {
                    let trim = if speed > 0.0 {
                        color_u8!(255, 190, 84, 255)
                    } else {
                        color_u8!(106, 231, 255, 255)
                    };
                    (color_u8!(55, 58, 72, 255), trim)
                }
            };
            draw_rectangle(
                x - 3.0,
                y - 3.0,
                platform.rect.w + 6.0,
                platform.rect.h + 6.0,
                color_u8!(6, 9, 18, 255),
            );
            draw_rectangle(x, y, platform.rect.w, platform.rect.h, fill);
            draw_rectangle(x, y, platform.rect.w, 6.0, trim);
            draw_rectangle(
                x,
                y + platform.rect.h - 5.0,
                platform.rect.w,
                5.0,
                color_u8!(0, 0, 0, 65),
            );

            if matches!(platform.kind, PlatformKind::Solid) {
                let mut tx = x + 18.0 - ((platform.rect.x * 0.17) % 34.0);
                while tx < x + platform.rect.w - 12.0 {
                    draw_line(
                        tx,
                        y + 8.0,
                        tx + 11.0,
                        y + 8.0,
                        2.0,
                        color_u8!(255, 255, 255, 34),
                    );
                    draw_rectangle(tx + 3.0, y + 13.0, 3.0, 3.0, sprite::ink());
                    tx += 34.0;
                }
            }

            if let PlatformKind::Conveyor { speed } = platform.kind {
                let arrow_color = if speed > 0.0 {
                    color_u8!(255, 228, 145, 255)
                } else {
                    color_u8!(170, 241, 255, 255)
                };
                let mut ax = x + 18.0 + ((time * speed.abs() * 0.18) % 34.0);
                while ax < x + platform.rect.w - 16.0 {
                    let dir = speed.signum();
                    draw_triangle(
                        vec2(ax + 9.0 * dir, y + 15.0),
                        vec2(ax - 8.0 * dir, y + 7.0),
                        vec2(ax - 8.0 * dir, y + 23.0),
                        arrow_color,
                    );
                    ax += 34.0;
                }
            }
        }

        for (index, hazard) in self.hazards.iter().enumerate() {
            if !visible(hazard.rect, camera_x) {
                continue;
            }
            let active = self.hazard_active(index, time);
            let x = hazard.rect.x - camera_x;
            let y = hazard.rect.y + y_offset;
            draw_rectangle(
                x - 8.0,
                y - 14.0,
                hazard.rect.w + 16.0,
                12.0,
                color_u8!(31, 35, 48, 255),
            );
            draw_rectangle(
                x,
                y,
                hazard.rect.w,
                hazard.rect.h,
                if active {
                    color_u8!(255, 69, 82, 210)
                } else {
                    color_u8!(92, 44, 55, 110)
                },
            );
            if active {
                draw_rectangle(
                    x + hazard.rect.w * 0.35,
                    y,
                    hazard.rect.w * 0.3,
                    hazard.rect.h,
                    color_u8!(255, 238, 180, 230),
                );
            }
        }
    }

    fn draw_background(&self, renderer: &SpriteRenderer, camera_x: f32, y_offset: f32, time: f32) {
        let section = self.section_name(camera_x + SCREEN_W * 0.5);
        let (sky, horizon, floor) = match section {
            "Legacy Jungle" => (
                color_u8!(8, 18, 29, 255),
                color_u8!(19, 55, 43, 255),
                color_u8!(25, 72, 48, 255),
            ),
            "CI/CD Factory" => (
                color_u8!(10, 15, 26, 255),
                color_u8!(35, 41, 57, 255),
                color_u8!(55, 58, 72, 255),
            ),
            _ => (
                color_u8!(13, 9, 24, 255),
                color_u8!(35, 24, 55, 255),
                color_u8!(52, 36, 74, 255),
            ),
        };
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, sky);
        draw_rectangle(0.0, 110.0 + y_offset, SCREEN_W, 100.0, horizon);
        draw_rectangle(0.0, 210.0 + y_offset, SCREEN_W, 275.0, floor);
        draw_rectangle(0.0, 0.0, SCREEN_W, 45.0, color_u8!(4, 7, 16, 145));

        if section == "Legacy Jungle" {
            draw_legacy_jungle(renderer, camera_x, y_offset, time);
        } else if section == "CI/CD Factory" {
            draw_ci_factory(renderer, camera_x, y_offset, time);
        } else {
            draw_production_core(renderer, camera_x, y_offset, time);
        }
    }
}

fn draw_legacy_jungle(renderer: &SpriteRenderer, camera_x: f32, y_offset: f32, time: f32) {
    renderer.draw_legacy_background(camera_x, y_offset, time);
    draw_circle(
        770.0 - camera_x * 0.025,
        78.0 + y_offset,
        52.0,
        color_u8!(94, 52, 121, 90),
    );
    for i in 0..22 {
        let x = parallax_x(i, 86.0, camera_x, 0.06);
        let y = 64.0 + ((i * 37) % 62) as f32;
        let blink = ((time * 2.0 + i as f32) as i32 % 3) == 0;
        draw_text(
            if blink { "fn" } else { "{}" },
            x,
            y + y_offset,
            13.0,
            color_u8!(110, 215, 146, 70),
        );
    }
    for i in 0..12 {
        let x = parallax_x(i, 180.0, camera_x, 0.13);
        let h = 112.0 + (i % 4) as f32 * 28.0;
        draw_rectangle(
            x + 44.0,
            330.0 - h + y_offset,
            28.0,
            h,
            color_u8!(12, 37, 31, 255),
        );
        draw_triangle(
            vec2(x - 8.0, 205.0 + y_offset),
            vec2(x + 58.0, 125.0 + y_offset),
            vec2(x + 126.0, 205.0 + y_offset),
            color_u8!(24, 73, 48, 255),
        );
        draw_triangle(
            vec2(x + 16.0, 258.0 + y_offset),
            vec2(x + 59.0, 174.0 + y_offset),
            vec2(x + 104.0, 258.0 + y_offset),
            color_u8!(31, 91, 56, 255),
        );
    }
    for i in 0..15 {
        let x = parallax_x(i, 132.0, camera_x, 0.27);
        draw_rectangle(
            x + 20.0,
            245.0 + y_offset,
            20.0,
            240.0,
            color_u8!(17, 49, 37, 255),
        );
        draw_line(
            x + 31.0,
            250.0 + y_offset,
            x - 8.0,
            318.0 + y_offset,
            5.0,
            color_u8!(21, 67, 44, 255),
        );
        draw_circle(
            x + 30.0,
            222.0 + y_offset,
            38.0,
            color_u8!(45, 111, 60, 255),
        );
        draw_circle(x + 62.0, 245.0 + y_offset, 33.0, color_u8!(38, 98, 57, 255));
        draw_rectangle(
            x + 7.0,
            306.0 + y_offset,
            48.0,
            24.0,
            color_u8!(17, 38, 36, 255),
        );
        draw_rectangle_lines(
            x + 7.0,
            306.0 + y_offset,
            48.0,
            24.0,
            2.0,
            color_u8!(92, 201, 118, 115),
        );
        draw_text(
            "v1",
            x + 17.0,
            324.0 + y_offset,
            14.0,
            color_u8!(142, 219, 122, 135),
        );
    }
}

fn draw_ci_factory(renderer: &SpriteRenderer, camera_x: f32, y_offset: f32, time: f32) {
    renderer.draw_factory_background(camera_x, y_offset, time);
    for i in 0..10 {
        let x = parallax_x(i, 160.0, camera_x, 0.09);
        draw_rectangle(x, 94.0 + y_offset, 104.0, 270.0, color_u8!(23, 28, 41, 255));
        draw_rectangle_lines(
            x + 8.0,
            114.0 + y_offset,
            88.0,
            228.0,
            3.0,
            color_u8!(64, 73, 92, 255),
        );
        for row in 0..5 {
            draw_rectangle(
                x + 20.0,
                134.0 + row as f32 * 38.0 + y_offset,
                64.0,
                12.0,
                color_u8!(35, 43, 58, 255),
            );
        }
    }
    for i in 0..8 {
        let x = parallax_x(i, 210.0, camera_x, 0.18);
        draw_line(
            x,
            190.0 + y_offset,
            x + 190.0,
            190.0 + y_offset,
            5.0,
            color_u8!(81, 88, 105, 255),
        );
        draw_line(
            x + 15.0,
            220.0 + y_offset,
            x + 176.0,
            324.0 + y_offset,
            4.0,
            color_u8!(81, 88, 105, 255),
        );
        draw_line(
            x + 176.0,
            220.0 + y_offset,
            x + 15.0,
            324.0 + y_offset,
            4.0,
            color_u8!(81, 88, 105, 255),
        );
        draw_rectangle(
            x + 24.0,
            342.0 + y_offset,
            136.0,
            36.0,
            color_u8!(31, 35, 48, 255),
        );
        draw_rectangle_lines(
            x + 24.0,
            342.0 + y_offset,
            136.0,
            36.0,
            3.0,
            color_u8!(255, 190, 84, 145),
        );
        let progress = ((time * 0.8 + i as f32 * 0.21).sin() * 0.5 + 0.5).clamp(0.1, 1.0);
        draw_rectangle(
            x + 34.0,
            355.0 + y_offset,
            112.0 * progress,
            8.0,
            color_u8!(255, 190, 84, 180),
        );
    }
    for i in 0..7 {
        let x = parallax_x(i, 180.0, camera_x, 0.31);
        draw_rectangle(
            x + 6.0,
            252.0 + y_offset,
            18.0,
            228.0,
            color_u8!(63, 67, 82, 255),
        );
        draw_rectangle(
            x + 78.0,
            238.0 + y_offset,
            20.0,
            242.0,
            color_u8!(63, 67, 82, 255),
        );
        draw_line(
            x + 15.0,
            282.0 + y_offset,
            x + 88.0,
            238.0 + y_offset,
            7.0,
            color_u8!(47, 52, 67, 255),
        );
        draw_circle(
            x + 50.0,
            212.0 + y_offset,
            18.0,
            color_u8!(255, 190, 84, 65),
        );
        draw_rectangle(
            x + 44.0,
            196.0 + y_offset,
            12.0,
            200.0,
            color_u8!(255, 88, 92, 45),
        );
    }
}

fn draw_production_core(renderer: &SpriteRenderer, camera_x: f32, y_offset: f32, time: f32) {
    renderer.draw_core_background(camera_x, y_offset, time);
    let pulse = (time * 3.0).sin() * 0.5 + 0.5;
    for i in 0..8 {
        let x = parallax_x(i, 176.0, camera_x, 0.08);
        draw_rectangle(x, 92.0 + y_offset, 116.0, 330.0, color_u8!(24, 17, 38, 255));
        draw_rectangle(
            x + 18.0,
            116.0 + y_offset,
            16.0,
            238.0,
            color_u8!(76, 45, 102, 255),
        );
        draw_rectangle(
            x + 72.0,
            138.0 + y_offset,
            12.0,
            208.0,
            color_u8!(76, 45, 102, 255),
        );
        draw_rectangle(
            x + 38.0,
            170.0 + y_offset,
            54.0,
            18.0,
            color_u8!(39, 27, 59, 255),
        );
    }
    for i in 0..6 {
        let x = parallax_x(i, 240.0, camera_x, 0.18);
        draw_circle_lines(
            x + 110.0,
            246.0 + y_offset,
            86.0,
            5.0,
            color_u8!(91, 52, 124, 190),
        );
        draw_circle_lines(
            x + 110.0,
            246.0 + y_offset,
            50.0 + pulse * 4.0,
            4.0,
            color_u8!(214, 126, 255, 170),
        );
        draw_line(
            x + 110.0,
            160.0 + y_offset,
            x + 110.0,
            332.0 + y_offset,
            4.0,
            color_u8!(214, 126, 255, 95),
        );
        draw_line(
            x + 24.0,
            246.0 + y_offset,
            x + 196.0,
            246.0 + y_offset,
            4.0,
            color_u8!(214, 126, 255, 95),
        );
        draw_rectangle(
            x + 83.0,
            219.0 + y_offset,
            54.0,
            54.0,
            color_u8!(14, 10, 25, 255),
        );
        draw_rectangle_lines(
            x + 83.0,
            219.0 + y_offset,
            54.0,
            54.0,
            3.0,
            color_u8!(255, 103, 97, 150),
        );
    }
    for i in 0..16 {
        let x = parallax_x(i, 78.0, camera_x, 0.34);
        let signal = ((time * 4.0 + i as f32) as i32 % 4) == 0;
        draw_rectangle(
            x + 8.0,
            306.0 + y_offset,
            44.0,
            118.0,
            color_u8!(31, 21, 49, 255),
        );
        draw_rectangle_lines(
            x + 8.0,
            306.0 + y_offset,
            44.0,
            118.0,
            2.0,
            color_u8!(99, 66, 132, 255),
        );
        draw_rectangle(
            x + 17.0,
            322.0 + y_offset,
            26.0,
            8.0,
            if signal {
                color_u8!(255, 103, 97, 190)
            } else {
                color_u8!(91, 52, 124, 150)
            },
        );
        draw_rectangle(
            x + 17.0,
            346.0 + y_offset,
            26.0,
            5.0,
            color_u8!(214, 126, 255, 120),
        );
        draw_rectangle(
            x + 17.0,
            364.0 + y_offset,
            26.0,
            5.0,
            color_u8!(214, 126, 255, 90),
        );
    }
}

fn parallax_x(index: i32, spacing: f32, camera_x: f32, factor: f32) -> f32 {
    index as f32 * spacing - (camera_x * factor).rem_euclid(spacing) - spacing
}

fn solid(x: f32, y: f32, w: f32, h: f32) -> Platform {
    Platform {
        rect: Rect::new(x, y, w, h),
        kind: PlatformKind::Solid,
    }
}

fn conveyor(x: f32, y: f32, w: f32, h: f32, speed: f32) -> Platform {
    Platform {
        rect: Rect::new(x, y, w, h),
        kind: PlatformKind::Conveyor { speed },
    }
}

fn visible(rect: Rect, camera_x: f32) -> bool {
    rect.x + rect.w > camera_x - 80.0 && rect.x < camera_x + SCREEN_W + 80.0
}

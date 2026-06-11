use crate::boss::Boss;
use crate::enemy::{Enemy, EnemyKind};
use crate::player::Player;
use crate::projectile::Weapon;
use crate::sprite;
use macroquad::prelude::*;
use macroquad::texture::{
    draw_texture_ex, load_image, DrawTextureParams, FilterMode, Image, Texture2D,
};

const HERO_SHEET: &str = "assets/open_gunner/OpenGunnerHeroVer2.png";
const ENEMIES_SHEET: &str = "assets/open_gunner/OpenGunnerEnemies.png";
const ENEMIES2_SHEET: &str = "assets/open_gunner/OpenGunnerEnemies2.png";
const MECHS_SHEET: &str = "assets/open_gunner/OpenGunnerMechs.png";
const FOREST_SHEET: &str = "assets/open_gunner/OpenGunnerForestTiles.png";
const STARTER_TILES_SHEET: &str = "assets/open_gunner/OpenGunnerStarterTiles.png";
const BARS_SHEET: &str = "assets/open_gunner/OpenGunnerBarsAndPanels.png";

pub struct SpriteRenderer {
    hero: Option<Texture2D>,
    enemies: Option<Texture2D>,
    enemies2: Option<Texture2D>,
    mechs: Option<Texture2D>,
    forest_tiles: Option<Texture2D>,
    starter_tiles: Option<Texture2D>,
    bars: Option<Texture2D>,
}

impl SpriteRenderer {
    pub async fn load() -> Self {
        Self {
            hero: load_sheet(HERO_SHEET).await,
            enemies: load_sheet(ENEMIES_SHEET).await,
            enemies2: load_sheet(ENEMIES2_SHEET).await,
            mechs: load_sheet(MECHS_SHEET).await,
            forest_tiles: load_sheet(FOREST_SHEET).await,
            starter_tiles: load_sheet(STARTER_TILES_SHEET).await,
            bars: load_sheet(BARS_SHEET).await,
        }
    }

    pub fn draw_player(
        &self,
        player: &Player,
        camera_x: f32,
        y_offset: f32,
        aim: Vec2,
        _gun_color: Color,
    ) -> bool {
        let Some(texture) = self.hero.as_ref() else {
            return false;
        };

        if player.crouching && player.on_ground {
            return false;
        }

        let dir = normalized_or_facing(aim, player.facing);
        let source = player_direction_frame(dir, player.facing);
        let moving_bob = if player.is_moving_horizontally() && player.on_ground {
            (get_time() as f32 * 16.0 + player.pos.x * 0.05).sin() * 1.4
        } else {
            0.0
        };
        let dest_size = vec2(58.0, 66.0);
        let x = player.pos.x - camera_x - dest_size.x * 0.5;
        let y = player.pos.y + y_offset - dest_size.y + moving_bob;

        draw_rectangle(
            player.pos.x - camera_x - 17.0,
            player.pos.y + y_offset - 4.0,
            34.0,
            5.0,
            color_u8!(0, 0, 0, 65),
        );
        draw_frame(texture, source, x, y, dest_size, false, WHITE);

        let cx = player.center().x - camera_x;
        let foot_y = player.pos.y + y_offset;
        draw_rectangle(cx - 6.0, foot_y - 42.0, 12.0, 5.0, sprite::ink());
        draw_rectangle(
            cx - 4.0,
            foot_y - 40.0,
            8.0,
            2.0,
            color_u8!(106, 231, 255, 230),
        );
        draw_rectangle(
            cx + 1.0,
            foot_y - 46.0,
            3.0,
            3.0,
            color_u8!(255, 213, 94, 230),
        );

        true
    }

    pub fn draw_enemy(&self, enemy: &Enemy, camera_x: f32, y_offset: f32) -> bool {
        if !enemy.alive {
            return true;
        }

        let rect = enemy.rect();
        let x = rect.x - camera_x;
        let y = rect.y + y_offset;
        let tint = if enemy.is_flashing() {
            WHITE
        } else {
            match enemy.kind {
                EnemyKind::BugCrawler => color_u8!(190, 255, 172, 255),
                EnemyKind::ExceptionBat => color_u8!(226, 190, 255, 255),
                EnemyKind::BuildTurret => color_u8!(255, 170, 127, 255),
                EnemyKind::TestBot => color_u8!(255, 229, 137, 255),
            }
        };

        match enemy.kind {
            EnemyKind::BugCrawler => {
                let Some(texture) = self.enemies2.as_ref() else {
                    return false;
                };
                let frame = if (get_time() as f32 * 9.0 + enemy.pos.x * 0.04).sin() > 0.0 {
                    atlas_cell(626.0, 835.0, 30.0, 30.0)
                } else {
                    atlas_cell(657.0, 835.0, 30.0, 30.0)
                };
                draw_frame(
                    texture,
                    frame,
                    x - 4.0,
                    y - 8.0,
                    vec2(44.0, 44.0),
                    false,
                    tint,
                );
                draw_line(x + 10.0, y + 1.0, x + 1.0, y - 7.0, 2.0, sprite::ink());
                draw_line(x + 25.0, y + 1.0, x + 34.0, y - 7.0, 2.0, sprite::ink());
                draw_circle(x + 1.0, y - 7.0, 2.0, color_u8!(255, 213, 94, 255));
                draw_circle(x + 34.0, y - 7.0, 2.0, color_u8!(255, 213, 94, 255));
                true
            }
            EnemyKind::ExceptionBat => {
                let Some(texture) = self.enemies.as_ref() else {
                    return false;
                };
                let frame = if (get_time() as f32 * 10.0 + enemy.pos.x * 0.02).sin() > 0.0 {
                    atlas_cell(47.0, 568.0, 50.0, 50.0)
                } else {
                    atlas_cell(99.0, 568.0, 50.0, 50.0)
                };
                let flap = (get_time() as f32 * 15.0 + enemy.pos.x * 0.03).sin() * 4.0;
                draw_frame(
                    texture,
                    frame,
                    x - 8.0,
                    y - 12.0 + flap,
                    vec2(54.0, 54.0),
                    false,
                    tint,
                );
                draw_text(
                    "!",
                    x + 17.0,
                    y + 13.0 + flap,
                    18.0,
                    color_u8!(255, 89, 94, 230),
                );
                true
            }
            EnemyKind::BuildTurret => {
                let Some(texture) = self.enemies.as_ref() else {
                    return false;
                };
                let flip_x = enemy.facing() > 0.0;
                let frame = atlas_cell(47.0, 318.0, 50.0, 50.0);
                draw_frame(
                    texture,
                    frame,
                    x - 5.0,
                    y - 3.0,
                    vec2(54.0, 54.0),
                    flip_x,
                    tint,
                );
                let barrel_y = y + 18.0;
                let barrel_x = if flip_x { x + rect.w - 1.0 } else { x - 16.0 };
                draw_rectangle(barrel_x, barrel_y, 18.0, 5.0, sprite::ink());
                draw_rectangle(
                    barrel_x,
                    barrel_y + 1.0,
                    18.0,
                    3.0,
                    color_u8!(255, 190, 84, 255),
                );
                true
            }
            EnemyKind::TestBot => {
                let Some(texture) = self.enemies2.as_ref() else {
                    return false;
                };
                let run = (get_time() as f32 * 14.0 + enemy.pos.x * 0.07).sin();
                let frame = if run > 0.0 {
                    atlas_cell(626.0, 870.0, 30.0, 30.0)
                } else {
                    atlas_cell(657.0, 870.0, 30.0, 30.0)
                };
                let flip_x = enemy.facing() < 0.0;
                draw_frame(
                    texture,
                    frame,
                    x - 4.0,
                    y - 8.0,
                    vec2(42.0, 42.0),
                    flip_x,
                    tint,
                );
                draw_rectangle(
                    x + 6.0,
                    y + 10.0,
                    rect.w - 12.0,
                    8.0,
                    color_u8!(18, 24, 33, 210),
                );
                draw_rectangle(x + 10.0, y + 12.0, 4.0, 3.0, color_u8!(255, 89, 94, 255));
                draw_rectangle(x + 20.0, y + 12.0, 4.0, 3.0, color_u8!(255, 89, 94, 255));
                true
            }
        }
    }

    pub fn draw_boss(&self, boss: &Boss, camera_x: f32, y_offset: f32) -> bool {
        let Some(texture) = self.mechs.as_ref() else {
            return false;
        };

        let rect = boss.rect();
        let x = rect.x - camera_x;
        let y = rect.y + y_offset;
        let time = get_time() as f32;
        let health_ratio = boss.health as f32 / boss.max_health as f32;
        let angry = health_ratio <= 0.34;
        let pulse = (time * if angry { 16.0 } else { 8.0 }).sin();
        let glow = (time * 3.4).sin() * 0.5 + 0.5;
        let frame = if boss.is_flashing() {
            atlas_cell(277.0, 696.0, 140.0, 108.0)
        } else if (time * if angry { 6.0 } else { 3.2 }).sin() > 0.45 {
            atlas_cell(136.0, 696.0, 140.0, 108.0)
        } else {
            atlas_cell(136.0, 584.0, 140.0, 108.0)
        };

        draw_circle_lines(
            x + rect.w * 0.5,
            y + 104.0,
            98.0 + glow * 8.0,
            4.0,
            color_u8!(214, 126, 255, 90),
        );
        draw_rectangle(
            x - 42.0,
            y - 16.0,
            rect.w + 84.0,
            rect.h + 24.0,
            color_u8!(8, 6, 14, 82),
        );

        let dest_size = vec2(252.0, 194.0);
        let tint = if boss.is_flashing() {
            WHITE
        } else if angry {
            color_u8!(255, 210, 218, 255)
        } else {
            color_u8!(218, 236, 255, 255)
        };
        draw_frame(texture, frame, x - 31.0, y + 6.0, dest_size, false, tint);

        let outline = sprite::ink();
        let armor = color_u8!(81, 67, 113, 230);
        let hot = if angry {
            color_u8!(255, 82, 92, 255)
        } else {
            color_u8!(255, 126, 112, 255)
        };
        let accent = color_u8!(214, 126, 255, 255);

        draw_rectangle(x + 24.0, y + 38.0, 142.0, 40.0, outline);
        draw_rectangle(x + 29.0, y + 43.0, 132.0, 30.0, color_u8!(30, 23, 47, 240));
        draw_rectangle(
            x + 34.0,
            y + 48.0,
            122.0 * health_ratio.max(0.06),
            4.0,
            accent,
        );
        draw_text("<<<<<<<", x + 36.0, y + 66.0, 18.0, hot);
        draw_rectangle(x + 55.0, y + 76.0, 80.0, 18.0, outline);
        draw_rectangle(x + 60.0, y + 80.0, 70.0, 9.0, color_u8!(255, 213, 94, 190));

        draw_circle(x + 95.0, y + 117.0, 24.0, outline);
        draw_circle(
            x + 95.0,
            y + 117.0,
            17.0 + glow * 2.0,
            color_u8!(214, 126, 255, 190),
        );
        draw_circle(x + 95.0, y + 117.0, 8.0, color_u8!(255, 238, 180, 230));

        draw_rectangle(x - 45.0, y + 104.0, 52.0, 16.0, outline);
        draw_rectangle(x - 42.0, y + 108.0, 52.0, 7.0, hot);
        draw_triangle(
            vec2(x - 50.0, y + 101.0),
            vec2(x - 72.0, y + 112.0),
            vec2(x - 50.0, y + 124.0),
            outline,
        );
        draw_circle(x - 72.0, y + 112.0, 8.0, hot);

        draw_rectangle(x + 181.0, y + 104.0, 54.0, 16.0, outline);
        draw_rectangle(x + 178.0, y + 108.0, 54.0, 7.0, hot);
        draw_triangle(
            vec2(x + 238.0, y + 101.0),
            vec2(x + 264.0, y + 112.0),
            vec2(x + 238.0, y + 124.0),
            outline,
        );
        draw_circle(x + 264.0, y + 112.0, 8.0, hot);

        draw_line(
            x + 55.0,
            y + 33.0,
            x + 35.0,
            y + 4.0 + pulse * 2.0,
            4.0,
            outline,
        );
        draw_line(
            x + 133.0,
            y + 33.0,
            x + 156.0,
            y + 4.0 - pulse * 2.0,
            4.0,
            outline,
        );
        draw_circle(x + 35.0, y + 4.0 + pulse * 2.0, 5.0, accent);
        draw_circle(x + 156.0, y + 4.0 - pulse * 2.0, 5.0, accent);

        draw_rectangle(x + 24.0, y + 154.0, 36.0, 42.0, outline);
        draw_rectangle(x + 29.0, y + 158.0, 26.0, 31.0, armor);
        draw_rectangle(x + 126.0, y + 154.0, 36.0, 42.0, outline);
        draw_rectangle(x + 131.0, y + 158.0, 26.0, 31.0, armor);
        draw_rectangle(x + 20.0, y + 188.0, 48.0, 11.0, outline);
        draw_rectangle(x + 118.0, y + 188.0, 48.0, 11.0, outline);
        draw_rectangle(x + 29.0, y + 186.0, 30.0, 5.0, hot);
        draw_rectangle(x + 127.0, y + 186.0, 30.0, 5.0, hot);

        if angry {
            draw_circle_lines(
                x + 95.0,
                y + 117.0,
                70.0 + pulse * 2.0,
                3.0,
                color_u8!(255, 103, 97, 155),
            );
        }

        true
    }

    pub fn draw_legacy_background(&self, camera_x: f32, y_offset: f32, _time: f32) {
        let Some(texture) = self.forest_tiles.as_ref() else {
            return;
        };

        for i in 0..7 {
            let x = i as f32 * 310.0 - (camera_x * 0.10).rem_euclid(310.0) - 150.0;
            let y = 104.0 + y_offset + (i % 2) as f32 * 18.0;
            let source = if i % 2 == 0 {
                Rect::new(1290.0, 145.0, 270.0, 330.0)
            } else {
                Rect::new(1644.0, 145.0, 250.0, 330.0)
            };
            draw_frame(
                texture,
                source,
                x,
                y,
                vec2(210.0, 252.0),
                false,
                color_u8!(119, 190, 121, 205),
            );
        }
    }

    pub fn draw_factory_background(&self, camera_x: f32, y_offset: f32, time: f32) {
        let Some(texture) = self.starter_tiles.as_ref() else {
            return;
        };

        let panel_sources = [
            atlas_cell(379.0, 255.0, 50.0, 50.0),
            atlas_cell(433.0, 311.0, 50.0, 50.0),
            atlas_cell(542.0, 255.0, 50.0, 50.0),
            atlas_cell(596.0, 255.0, 50.0, 50.0),
        ];

        for i in 0..16 {
            let x = i as f32 * 96.0 - (camera_x * 0.16).rem_euclid(96.0) - 84.0;
            let y = 128.0 + y_offset + (i % 3) as f32 * 46.0;
            let source = panel_sources[i as usize % panel_sources.len()];
            let lit = ((time * 3.0 + i as f32) as i32 % 3) == 0;
            draw_frame(
                texture,
                source,
                x,
                y,
                vec2(46.0, 46.0),
                false,
                if lit {
                    color_u8!(255, 224, 156, 180)
                } else {
                    color_u8!(150, 192, 185, 125)
                },
            );
        }
    }

    pub fn draw_core_background(&self, camera_x: f32, y_offset: f32, time: f32) {
        let Some(texture) = self.bars.as_ref() else {
            return;
        };

        for i in 0..9 {
            let x = i as f32 * 156.0 - (camera_x * 0.14).rem_euclid(156.0) - 80.0;
            let y = 128.0 + y_offset + (i % 2) as f32 * 74.0;
            let source = if i % 2 == 0 {
                Rect::new(22.0, 173.0, 54.0, 66.0)
            } else {
                Rect::new(197.0, 176.0, 94.0, 34.0)
            };
            let pulse = ((time * 4.0 + i as f32) as i32 % 4) == 0;
            draw_frame(
                texture,
                source,
                x,
                y,
                if i % 2 == 0 {
                    vec2(42.0, 70.0)
                } else {
                    vec2(112.0, 40.0)
                },
                false,
                if pulse {
                    color_u8!(255, 160, 142, 190)
                } else {
                    color_u8!(218, 173, 255, 135)
                },
            );
        }
    }
}

async fn load_sheet(path: &str) -> Option<Texture2D> {
    match load_image(path).await {
        Ok(mut image) => {
            strip_sheet_matte(&mut image);
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            Some(texture)
        }
        Err(_err) => None,
    }
}

fn strip_sheet_matte(image: &mut Image) {
    for pixel in image.get_image_data_mut().iter_mut() {
        if is_sheet_matte(pixel) {
            pixel[3] = 0;
        }
    }
}

fn is_sheet_matte(pixel: &[u8; 4]) -> bool {
    let r = pixel[0];
    let g = pixel[1];
    let b = pixel[2];
    (70..=95).contains(&r) && (65..=92).contains(&g) && (105..=145).contains(&b)
}

fn draw_frame(
    texture: &Texture2D,
    source: Rect,
    x: f32,
    y: f32,
    dest_size: Vec2,
    flip_x: bool,
    tint: Color,
) {
    draw_texture_ex(
        texture,
        x,
        y,
        tint,
        DrawTextureParams {
            source: Some(source),
            dest_size: Some(dest_size),
            flip_x,
            ..Default::default()
        },
    );
}

fn atlas_cell(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect::new(x + 1.0, y + 1.0, w - 2.0, h - 2.0)
}

fn normalized_or_facing(aim: Vec2, facing: f32) -> Vec2 {
    if aim.length_squared() > 0.01 {
        aim.normalize()
    } else {
        vec2(facing, 0.0)
    }
}

fn player_direction_frame(dir: Vec2, facing: f32) -> Rect {
    if dir.y < -0.35 {
        if dir.x > 0.35 {
            atlas_cell(81.0, 676.0, 50.0, 57.0)
        } else if dir.x < -0.35 {
            atlas_cell(249.0, 676.0, 50.0, 57.0)
        } else if facing < 0.0 {
            atlas_cell(193.0, 676.0, 50.0, 57.0)
        } else {
            atlas_cell(137.0, 676.0, 50.0, 57.0)
        }
    } else if dir.y > 0.35 {
        if dir.x > 0.35 {
            atlas_cell(81.0, 739.0, 50.0, 57.0)
        } else if dir.x < -0.35 {
            atlas_cell(249.0, 739.0, 50.0, 57.0)
        } else if facing < 0.0 {
            atlas_cell(193.0, 739.0, 50.0, 57.0)
        } else {
            atlas_cell(137.0, 739.0, 50.0, 57.0)
        }
    } else if dir.x < -0.08 {
        atlas_cell(305.0, 676.0, 50.0, 57.0)
    } else {
        atlas_cell(25.0, 676.0, 50.0, 57.0)
    }
}

#[allow(dead_code)]
fn weapon_tint(weapon: Weapon) -> Color {
    match weapon {
        Weapon::PatchRifle => color_u8!(106, 231, 255, 255),
        Weapon::SpreadDiff => color_u8!(255, 213, 94, 255),
        Weapon::RefactorBeam => color_u8!(176, 255, 155, 255),
        Weapon::HotfixSmg => color_u8!(255, 120, 167, 255),
    }
}

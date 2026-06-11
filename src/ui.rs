use crate::boss::Boss;
use crate::level::{SCREEN_H, SCREEN_W};
use crate::player::Player;
use macroquad::prelude::*;

pub fn draw_hud(score: i32, player: &Player, section: &str, boss: &Boss) {
    draw_rectangle(0.0, 0.0, SCREEN_W, 46.0, color_u8!(5, 8, 17, 230));
    draw_text(
        format!("SCORE {:06}", score),
        18.0,
        30.0,
        24.0,
        color_u8!(255, 245, 212, 255),
    );
    draw_text(
        format!("LIVES {}", player.lives),
        206.0,
        30.0,
        24.0,
        color_u8!(255, 245, 212, 255),
    );

    draw_text("HP", 322.0, 30.0, 24.0, color_u8!(255, 245, 212, 255));
    for i in 0..player.max_health {
        let filled = i < player.health;
        draw_rectangle(
            360.0 + i as f32 * 20.0,
            15.0,
            15.0,
            18.0,
            if filled {
                color_u8!(119, 255, 150, 255)
            } else {
                color_u8!(52, 44, 54, 255)
            },
        );
    }

    draw_text(
        player.weapon.name(),
        508.0,
        30.0,
        24.0,
        color_u8!(106, 231, 255, 255),
    );
    draw_text(section, 735.0, 30.0, 24.0, color_u8!(255, 213, 94, 255));

    if player.shield_timer > 0.0 {
        draw_text(
            &format!("SHIELD {:.0}s", player.shield_timer.ceil()),
            508.0,
            44.0,
            14.0,
            color_u8!(147, 176, 255, 255),
        );
    }

    if boss.active && !boss.defeated {
        let w = 420.0;
        let pct = boss.health as f32 / boss.max_health as f32;
        draw_rectangle(270.0, 58.0, w, 18.0, color_u8!(5, 8, 17, 230));
        draw_rectangle(
            274.0,
            62.0,
            (w - 8.0) * pct,
            10.0,
            color_u8!(214, 126, 255, 255),
        );
        draw_text(
            "MERGE CONFLICT MECH",
            340.0,
            98.0,
            24.0,
            color_u8!(255, 245, 212, 255),
        );
    }
}

pub fn draw_start() {
    draw_menu_backdrop();
    centered("PATCH FORCE", 110.0, 66, color_u8!(255, 245, 212, 255));
    centered(
        "Runtime Rebellion",
        168.0,
        40,
        color_u8!(106, 231, 255, 255),
    );
    centered(
        "A tiny dev commando run-and-gun",
        236.0,
        25,
        color_u8!(255, 213, 94, 255),
    );
    centered("ENTER  Start", 330.0, 28, color_u8!(255, 245, 212, 255));
    centered("I  Instructions", 370.0, 24, color_u8!(176, 255, 155, 255));
}

pub fn draw_instructions() {
    draw_menu_backdrop();
    centered("INSTRUCTIONS", 72.0, 48, color_u8!(255, 245, 212, 255));
    let lines = [
        "A/D or Left/Right: move",
        "W, Space, or Up: jump",
        "S or Down: crouch",
        "J or Left Click: shoot",
        "Aim with keyboard directions for eight-way fire",
        "Move the mouse to enable crosshair aim",
        "K: return to keyboard aim",
        "R: restart    Escape: pause",
        "Collect weapon, health, and Test Shield pickups",
    ];
    for (i, line) in lines.iter().enumerate() {
        centered(
            line,
            132.0 + i as f32 * 32.0,
            24,
            color_u8!(224, 234, 255, 255),
        );
    }
    centered("ENTER  Start", 454.0, 28, color_u8!(255, 213, 94, 255));
}

pub fn draw_pause() {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, color_u8!(5, 8, 17, 170));
    centered("PAUSED", 220.0, 54, color_u8!(255, 245, 212, 255));
    centered("Escape to resume", 286.0, 26, color_u8!(106, 231, 255, 255));
    centered("R to restart run", 322.0, 22, color_u8!(255, 213, 94, 255));
}

pub fn draw_damage_flash(timer: f32) {
    let alpha = (timer / 0.22).clamp(0.0, 1.0);
    draw_rectangle(
        0.0,
        0.0,
        SCREEN_W,
        SCREEN_H,
        Color::new(255.0 / 255.0, 69.0 / 255.0, 82.0 / 255.0, 0.18 * alpha),
    );
    draw_rectangle_lines(
        6.0,
        52.0,
        SCREEN_W - 12.0,
        SCREEN_H - 58.0,
        6.0,
        Color::new(255.0 / 255.0, 103.0 / 255.0, 97.0 / 255.0, 0.55 * alpha),
    );
}

pub fn draw_game_over(score: i32) {
    draw_menu_backdrop();
    centered("DEPLOY FAILED", 145.0, 58, color_u8!(255, 103, 97, 255));
    centered(
        &format!("Final score {:06}", score),
        236.0,
        30,
        color_u8!(255, 245, 212, 255),
    );
    centered(
        "R or Enter  Restart",
        330.0,
        28,
        color_u8!(255, 213, 94, 255),
    );
}

pub fn draw_victory(score: i32) {
    draw_menu_backdrop();
    centered("PATCH LANDED", 130.0, 60, color_u8!(176, 255, 155, 255));
    centered(
        "The production codebase is stable again.",
        212.0,
        28,
        color_u8!(255, 245, 212, 255),
    );
    centered(
        &format!("Final score {:06}", score),
        270.0,
        30,
        color_u8!(106, 231, 255, 255),
    );
    centered(
        "R or Enter  Play again",
        360.0,
        28,
        color_u8!(255, 213, 94, 255),
    );
}

pub fn draw_tip(text: &str, y: f32, color: Color) {
    let size = 20;
    let dims = measure_text(text, None, size, 1.0);
    let pad_x = 18.0;
    let pad_y = 9.0;
    let w = dims.width + pad_x * 2.0;
    let h = size as f32 + pad_y * 1.4;
    let x = (SCREEN_W - w) * 0.5;

    draw_rectangle(
        x - 3.0,
        y - size as f32 - 7.0,
        w + 6.0,
        h + 6.0,
        color_u8!(4, 7, 16, 210),
    );
    draw_rectangle(x, y - size as f32 - 4.0, w, h, color_u8!(20, 25, 42, 225));
    draw_rectangle(x, y - size as f32 - 4.0, 5.0, h, color);
    draw_text(text, x + pad_x, y, size as f32, color);
}

pub fn draw_notice(text: &str, index: usize, timer: f32, color: Color) {
    let size = 18;
    let alpha = timer.min(1.0).clamp(0.0, 1.0);
    let dims = measure_text(text, None, size, 1.0);
    let pad_x = 14.0;
    let w = dims.width + pad_x * 2.0;
    let h = 30.0;
    let x = SCREEN_W - w - 22.0;
    let y = 112.0 + index as f32 * 36.0;
    let bg = Color::new(5.0 / 255.0, 8.0 / 255.0, 17.0 / 255.0, 0.72 * alpha);
    let fg = Color::new(color.r, color.g, color.b, alpha);

    draw_rectangle(x - 3.0, y - 3.0, w + 6.0, h + 6.0, bg);
    draw_rectangle(
        x,
        y,
        w,
        h,
        Color::new(20.0 / 255.0, 25.0 / 255.0, 42.0 / 255.0, 0.86 * alpha),
    );
    draw_rectangle(x, y, 5.0, h, fg);
    draw_text(text, x + pad_x, y + 21.0, size as f32, fg);
}

fn draw_menu_backdrop() {
    clear_background(color_u8!(11, 16, 32, 255));
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, color_u8!(11, 16, 32, 255));
    for i in 0..18 {
        let x = i as f32 * 68.0;
        let h = 80.0 + ((i * 31) % 140) as f32;
        draw_rectangle(x, SCREEN_H - h, 38.0, h, color_u8!(18, 29, 42, 255));
    }
    draw_rectangle(0.0, 420.0, SCREEN_W, 120.0, color_u8!(31, 35, 48, 255));
    draw_line(
        0.0,
        420.0,
        SCREEN_W,
        420.0,
        5.0,
        color_u8!(106, 231, 255, 255),
    );
}

fn centered(text: &str, y: f32, size: u16, color: Color) {
    let dims = measure_text(text, None, size, 1.0);
    draw_text(text, (SCREEN_W - dims.width) * 0.5, y, size as f32, color);
}

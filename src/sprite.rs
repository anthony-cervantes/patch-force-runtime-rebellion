use macroquad::prelude::*;

pub fn ink() -> Color {
    color_u8!(5, 8, 17, 255)
}

pub fn deep_shadow() -> Color {
    color_u8!(11, 16, 32, 255)
}

pub fn draw_outlined_rect(x: f32, y: f32, w: f32, h: f32, fill: Color) {
    draw_rectangle(x - 3.0, y - 3.0, w + 6.0, h + 6.0, ink());
    draw_rectangle(x, y, w, h, fill);
}

pub fn draw_beveled_rect(x: f32, y: f32, w: f32, h: f32, fill: Color, hi: Color, lo: Color) {
    draw_outlined_rect(x, y, w, h, fill);
    draw_rectangle(x, y, w, 3.0, hi);
    draw_rectangle(x, y + h - 3.0, w, 3.0, lo);
}

pub fn facing_x(base_x: f32, total_w: f32, local_x: f32, w: f32, facing: f32) -> f32 {
    if facing >= 0.0 {
        base_x + local_x
    } else {
        base_x + total_w - local_x - w
    }
}

pub fn draw_facing_rect(
    base_x: f32,
    base_y: f32,
    total_w: f32,
    local: Rect,
    facing: f32,
    color: Color,
) {
    draw_rectangle(
        facing_x(base_x, total_w, local.x, local.w, facing),
        base_y + local.y,
        local.w,
        local.h,
        color,
    );
}

pub fn draw_facing_outlined_rect(
    base_x: f32,
    base_y: f32,
    total_w: f32,
    local: Rect,
    facing: f32,
    color: Color,
) {
    draw_outlined_rect(
        facing_x(base_x, total_w, local.x, local.w, facing),
        base_y + local.y,
        local.w,
        local.h,
        color,
    );
}

pub fn draw_glint(x: f32, y: f32, w: f32, color: Color) {
    draw_rectangle(x, y, w, 2.0, color);
    draw_rectangle(x + 2.0, y - 2.0, 2.0, 6.0, color);
}

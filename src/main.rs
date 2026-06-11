mod audio;
mod boss;
mod enemy;
mod game;
mod level;
mod particle;
mod pickup;
mod player;
mod projectile;
mod sprite;
mod sprite_renderer;
mod ui;

use audio::AudioBank;
use game::Game;
use macroquad::prelude::*;
use sprite_renderer::SpriteRenderer;

fn window_conf() -> Conf {
    Conf {
        window_title: "Patch Force: Runtime Rebellion".to_string(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let renderer = SpriteRenderer::load().await;
    let audio = AudioBank::load().await;
    let mut game = Game::new(renderer, audio);

    loop {
        let dt = get_frame_time().clamp(0.0, 1.0 / 30.0);
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}

use crate::audio::{AudioBank, Sfx};
use crate::boss::Boss;
use crate::enemy::{Enemy, EnemyKind};
use crate::level::{Level, SCREEN_H, SCREEN_W};
use crate::particle::Particle;
use crate::pickup::{Pickup, PickupKind};
use crate::player::Player;
use crate::projectile::{Projectile, ProjectileKind, ProjectileOwner, Weapon};
use crate::sprite_renderer::SpriteRenderer;
use crate::ui;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Screen {
    Start,
    Instructions,
    Playing,
    Paused,
    GameOver,
    Victory,
}

struct Notice {
    text: String,
    timer: f32,
    color: Color,
}

pub struct Game {
    screen: Screen,
    level: Level,
    player: Player,
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>,
    pickups: Vec<Pickup>,
    particles: Vec<Particle>,
    boss: Boss,
    renderer: SpriteRenderer,
    camera_x: f32,
    score: i32,
    elapsed: f32,
    shake_timer: f32,
    shake_strength: f32,
    damage_flash_timer: f32,
    checkpoint: Vec2,
    checkpoint_stage: u8,
    encounter_stage: u8,
    notices: Vec<Notice>,
    audio: AudioBank,
    mouse_seen: bool,
    mouse_aim_active: bool,
    mouse_tip_shown: bool,
    last_mouse_screen: Vec2,
}

impl Game {
    pub fn new(renderer: SpriteRenderer, audio: AudioBank) -> Self {
        let level = Level::new();
        let checkpoint = vec2(90.0, 480.0);
        Self {
            screen: Screen::Start,
            level,
            player: Player::new(checkpoint),
            enemies: Vec::new(),
            projectiles: Vec::new(),
            pickups: Vec::new(),
            particles: Vec::new(),
            boss: Boss::new(),
            renderer,
            camera_x: 0.0,
            score: 0,
            elapsed: 0.0,
            shake_timer: 0.0,
            shake_strength: 0.0,
            damage_flash_timer: 0.0,
            checkpoint,
            checkpoint_stage: 0,
            encounter_stage: 0,
            notices: Vec::new(),
            audio,
            mouse_seen: false,
            mouse_aim_active: false,
            mouse_tip_shown: false,
            last_mouse_screen: Vec2::ZERO,
        }
        .with_run_state()
    }

    pub fn update(&mut self, dt: f32) {
        match self.screen {
            Screen::Start => {
                if is_key_pressed(KeyCode::I) {
                    self.screen = Screen::Instructions;
                } else if is_key_pressed(KeyCode::Enter) {
                    self.reset_run();
                    self.screen = Screen::Playing;
                }
            }
            Screen::Instructions => {
                if is_key_pressed(KeyCode::Escape) {
                    self.screen = Screen::Start;
                } else if is_key_pressed(KeyCode::Enter) {
                    self.reset_run();
                    self.screen = Screen::Playing;
                }
            }
            Screen::Paused => {
                if is_key_pressed(KeyCode::Escape) {
                    self.screen = Screen::Playing;
                } else if is_key_pressed(KeyCode::R) {
                    self.reset_run();
                    self.screen = Screen::Playing;
                }
            }
            Screen::GameOver | Screen::Victory => {
                if is_key_pressed(KeyCode::R) || is_key_pressed(KeyCode::Enter) {
                    self.reset_run();
                    self.screen = Screen::Playing;
                }
            }
            Screen::Playing => self.update_playing(dt),
        }
    }

    pub fn draw(&self) {
        show_mouse(!(self.screen == Screen::Playing && self.mouse_aim_active));
        set_camera(&virtual_screen_camera());

        let (shake_x, shake_y) = if self.shake_timer > 0.0 {
            (
                gen_range(-self.shake_strength, self.shake_strength),
                gen_range(-self.shake_strength, self.shake_strength),
            )
        } else {
            (0.0, 0.0)
        };
        let camera_x = self.camera_x + shake_x;

        match self.screen {
            Screen::Start => ui::draw_start(),
            Screen::Instructions => ui::draw_instructions(),
            Screen::GameOver => ui::draw_game_over(self.score),
            Screen::Victory => ui::draw_victory(self.score),
            Screen::Playing | Screen::Paused => {
                self.level
                    .draw(&self.renderer, camera_x, shake_y, self.elapsed);
                for pickup in &self.pickups {
                    pickup.draw(camera_x, shake_y);
                }
                for enemy in &self.enemies {
                    enemy.draw(&self.renderer, camera_x, shake_y);
                }
                self.boss.draw(&self.renderer, camera_x, shake_y);
                let aim = self.current_aim_direction();
                self.player.draw(&self.renderer, camera_x, shake_y, aim);
                for projectile in &self.projectiles {
                    projectile.draw(camera_x, shake_y);
                }
                for particle in &self.particles {
                    particle.draw(camera_x, shake_y);
                }
                ui::draw_hud(
                    self.score,
                    &self.player,
                    self.level.section_name(self.player.pos.x),
                    &self.boss,
                );
                if self.damage_flash_timer > 0.0 {
                    ui::draw_damage_flash(self.damage_flash_timer);
                }
                if self.screen == Screen::Playing {
                    self.draw_context_prompts();
                }
                self.draw_notices();
                if self.screen == Screen::Playing {
                    self.draw_mouse_crosshair();
                }
                if self.screen == Screen::Paused {
                    ui::draw_pause();
                }
            }
        }
        set_default_camera();
    }

    fn update_playing(&mut self, dt: f32) {
        if is_key_pressed(KeyCode::Escape) {
            self.screen = Screen::Paused;
            return;
        }
        if is_key_pressed(KeyCode::R) {
            self.reset_run();
            return;
        }

        self.update_mouse_aim_state();
        self.elapsed += dt;
        self.shake_timer = (self.shake_timer - dt).max(0.0);
        self.damage_flash_timer = (self.damage_flash_timer - dt).max(0.0);
        if self.shake_timer == 0.0 {
            self.shake_strength = 0.0;
        }

        self.player.update(dt, &self.level);
        self.update_player_mouse_facing();
        if self.player.pos.x > 2320.0 && self.checkpoint_stage < 1 {
            self.checkpoint = vec2(2260.0, 480.0);
            self.checkpoint_stage = 1;
            self.audio.play(Sfx::Checkpoint);
            self.push_notice(
                "Checkpoint: CI/CD Factory",
                color_u8!(255, 213, 94, 255),
                2.6,
            );
        }
        if self.player.pos.x > self.level.boss_start + 120.0 && self.checkpoint_stage < 2 {
            self.checkpoint = vec2(self.level.boss_start + 80.0, 480.0);
            self.checkpoint_stage = 2;
            self.audio.play(Sfx::Checkpoint);
            self.push_notice(
                "Checkpoint: Production Core",
                color_u8!(214, 126, 255, 255),
                2.6,
            );
        }

        if self.player.pos.x > self.level.boss_start {
            if !self.boss.active {
                self.player.heal(3);
                self.player.shield(2.5);
                self.audio.play(Sfx::Boss);
                self.push_notice(
                    "Merge Conflict Mech deployed",
                    color_u8!(255, 103, 97, 255),
                    3.0,
                );
            }
            self.boss.active = true;
            if self.checkpoint_stage < 2 {
                self.checkpoint = vec2(self.level.boss_start + 80.0, 480.0);
                self.checkpoint_stage = 2;
            }
            self.player.pos.x = self.player.pos.x.max(self.level.boss_start + 34.0);
        }

        self.handle_shooting();
        self.update_encounter_waves();
        self.update_enemies(dt);
        self.update_boss(dt);
        self.update_projectiles(dt);
        self.update_pickups(dt);
        self.update_particles(dt);
        self.update_notices(dt);
        self.handle_player_hazards();

        self.camera_x =
            (self.player.pos.x - SCREEN_W * 0.38).clamp(0.0, self.level.width - SCREEN_W);
        if self.boss.active && !self.boss.defeated && self.player.pos.x >= self.level.boss_start {
            self.camera_x = self.level.boss_start.min(self.level.width - SCREEN_W);
        }

        if self.player.lives <= 0 {
            self.screen = Screen::GameOver;
        }
        if self.boss.defeated {
            self.score += 5000;
            self.audio.play(Sfx::Victory);
            self.screen = Screen::Victory;
        }
    }

    fn handle_shooting(&mut self) {
        let shooting = is_key_down(KeyCode::J)
            || is_key_pressed(KeyCode::J)
            || is_mouse_button_down(MouseButton::Left)
            || is_mouse_button_pressed(MouseButton::Left);
        if !shooting || self.player.fire_cooldown > 0.0 {
            return;
        }

        let aim = self.current_aim_direction();
        let muzzle = self.player.muzzle_pos(aim);
        let weapon = self.player.weapon;
        self.player.fire_cooldown = weapon.fire_delay();
        self.projectiles
            .extend(Projectile::player(muzzle, aim, weapon));
        self.audio.play(Sfx::Shoot);
        self.particles.push(Particle::new(
            muzzle,
            aim * 70.0,
            0.12,
            6.0,
            color_u8!(255, 245, 212, 255),
        ));
    }

    fn update_mouse_aim_state(&mut self) {
        if self.mouse_aim_active && is_key_pressed(KeyCode::K) {
            self.mouse_aim_active = false;
            self.push_notice("Keyboard aim restored", color_u8!(255, 213, 94, 255), 2.2);
            return;
        }

        let (mx, my) = mouse_position();
        let mouse = vec2(mx, my);
        if !self.mouse_seen {
            self.last_mouse_screen = mouse;
            self.mouse_seen = true;
            return;
        }

        let moved = (mouse - self.last_mouse_screen).length_squared() > 0.6;
        if moved
            || is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_down(MouseButton::Left)
        {
            if !self.mouse_aim_active && !self.mouse_tip_shown {
                self.push_notice(
                    "Mouse aim online - K for keyboard",
                    color_u8!(106, 231, 255, 255),
                    2.8,
                );
                self.mouse_tip_shown = true;
            }
            self.mouse_aim_active = true;
        }
        self.last_mouse_screen = mouse;
    }

    fn update_player_mouse_facing(&mut self) {
        if let Some(aim) = self.mouse_aim_direction() {
            if aim.x.abs() > 0.18 {
                self.player.facing = aim.x.signum();
            }
        }
    }

    fn current_aim_direction(&self) -> Vec2 {
        self.mouse_aim_direction()
            .unwrap_or_else(|| self.player.aim_direction())
    }

    fn mouse_aim_direction(&self) -> Option<Vec2> {
        if !self.mouse_aim_active {
            return None;
        }

        let (mx, my) = mouse_position();
        if mx < 0.0 || my < 0.0 || mx > screen_width() || my > screen_height() {
            return None;
        }

        let mouse = self.virtual_mouse_position();
        let world_mouse = vec2(mouse.x + self.camera_x, mouse.y);
        let aim = world_mouse - self.player.center();
        if aim.length() > 18.0 {
            Some(aim.normalize_or_zero())
        } else {
            None
        }
    }

    fn draw_mouse_crosshair(&self) {
        if !self.mouse_aim_active {
            return;
        }

        let (mx, my) = mouse_position();
        if mx < 0.0 || my < 0.0 || mx > screen_width() || my > screen_height() {
            return;
        }

        let mouse = self.virtual_mouse_position();
        let aim = self.current_aim_direction();
        let dir = if aim.length_squared() > 0.0 {
            aim
        } else {
            vec2(1.0, 0.0)
        };
        let perp = vec2(-dir.y, dir.x);
        let center = mouse;
        let color = weapon_color(self.player.weapon);
        let shadow = color_u8!(4, 7, 16, 230);

        draw_circle_lines(mouse.x, mouse.y, 15.0, 4.0, shadow);
        draw_circle_lines(mouse.x, mouse.y, 15.0, 2.0, color);
        draw_line_points(center - dir * 30.0, center - dir * 10.0, 4.0, shadow);
        draw_line_points(center + dir * 10.0, center + dir * 30.0, 4.0, shadow);
        draw_line_points(center - perp * 24.0, center - perp * 9.0, 4.0, shadow);
        draw_line_points(center + perp * 9.0, center + perp * 24.0, 4.0, shadow);
        draw_line_points(center - dir * 30.0, center - dir * 10.0, 2.0, color);
        draw_line_points(center + dir * 10.0, center + dir * 30.0, 2.0, color);
        draw_line_points(center - perp * 24.0, center - perp * 9.0, 2.0, color);
        draw_line_points(center + perp * 9.0, center + perp * 24.0, 2.0, color);
        draw_circle(mouse.x, mouse.y, 4.0, shadow);
        draw_circle(mouse.x, mouse.y, 2.0, color_u8!(255, 255, 255, 235));
    }

    fn draw_context_prompts(&self) {
        if self.elapsed < 6.0 && self.player.pos.x < 420.0 {
            ui::draw_tip(
                "Move right. Jump with W / Space.",
                82.0,
                color_u8!(255, 245, 212, 255),
            );
        } else if self.player.pos.x < 930.0 {
            ui::draw_tip(
                "Grab Spread Diff, then fire with J or click.",
                82.0,
                color_u8!(255, 213, 94, 255),
            );
        } else if (2140.0..3950.0).contains(&self.player.pos.x) {
            ui::draw_tip(
                "Red beams cycle. Wait for the warning blink.",
                82.0,
                color_u8!(255, 213, 94, 255),
            );
        } else if self.boss.active && !self.boss.defeated {
            ui::draw_tip(
                "Boss phase shifts as its health drops.",
                118.0,
                color_u8!(214, 126, 255, 255),
            );
        }
    }

    fn draw_notices(&self) {
        for (index, notice) in self.notices.iter().enumerate() {
            ui::draw_notice(&notice.text, index, notice.timer, notice.color);
        }
    }

    fn virtual_mouse_position(&self) -> Vec2 {
        let (mx, my) = mouse_position();
        let scale_x = SCREEN_W / screen_width().max(1.0);
        let scale_y = SCREEN_H / screen_height().max(1.0);
        vec2(mx * scale_x, my * scale_y)
    }

    fn update_enemies(&mut self, dt: f32) {
        let player_pos = self.player.center();
        let mut spawned_shots = Vec::new();
        let mut took_damage = false;
        let mut took_life_hit = false;
        for enemy in &mut self.enemies {
            if enemy.rect().x > self.camera_x + SCREEN_W + 260.0
                || enemy.rect().x + enemy.rect().w < self.camera_x - 360.0
            {
                continue;
            }
            enemy.update(dt, player_pos, &self.level, &mut spawned_shots);
            if enemy.alive && enemy.rect().overlaps(&self.player.rect()) {
                let before_health = self.player.health;
                let before_lives = self.player.lives;
                let life_lost = self.player.take_damage(1, self.checkpoint);
                took_damage |= life_lost
                    || self.player.health != before_health
                    || self.player.lives != before_lives;
                took_life_hit |= life_lost;
            }
        }
        self.projectiles.extend(spawned_shots);
        if took_damage {
            self.player_damage_feedback(took_life_hit, "Enemy collision");
        }
    }

    fn update_boss(&mut self, dt: f32) {
        let mut spawned = Vec::new();
        let mut shake = 0.0_f32;
        self.boss.update(
            dt,
            self.player.center(),
            &mut self.projectiles,
            &mut spawned,
            &mut self.particles,
            &mut shake,
        );
        if shake > 0.0 {
            self.shake(shake, 13.0);
        }
        self.enemies.extend(spawned);
    }

    fn update_encounter_waves(&mut self) {
        if self.encounter_stage == 0 && self.player.pos.x > 1260.0 {
            self.encounter_stage = 1;
            self.push_notice("Runtime ambush incoming", color_u8!(255, 213, 94, 255), 2.2);
            self.enemies
                .push(Enemy::new(EnemyKind::ExceptionBat, 1530.0, 230.0, None));
            self.enemies
                .push(Enemy::new(EnemyKind::BugCrawler, 1690.0, 456.0, None));
        }

        if self.encounter_stage == 1 && self.player.pos.x > 2860.0 {
            self.encounter_stage = 2;
            self.push_notice(
                "Factory regression wave",
                color_u8!(255, 120, 167, 255),
                2.2,
            );
            self.enemies
                .push(Enemy::new(EnemyKind::TestBot, 3330.0, 398.0, None));
            self.enemies
                .push(Enemy::new(EnemyKind::ExceptionBat, 3520.0, 250.0, None));
        }

        if self.encounter_stage == 2 && self.player.pos.x > 4200.0 && !self.boss.active {
            self.encounter_stage = 3;
            self.push_notice("Core breach detected", color_u8!(214, 126, 255, 255), 2.2);
            self.enemies
                .push(Enemy::new(EnemyKind::BugCrawler, 4570.0, 456.0, None));
            self.enemies
                .push(Enemy::new(EnemyKind::ExceptionBat, 4840.0, 265.0, None));
        }
    }

    fn update_projectiles(&mut self, dt: f32) {
        for projectile in &mut self.projectiles {
            projectile.update(dt);
            if projectile.pos.x < -200.0
                || projectile.pos.x > self.level.width + 200.0
                || projectile.pos.y < -200.0
                || projectile.pos.y > SCREEN_H + 200.0
            {
                projectile.alive = false;
            }
        }

        self.handle_projectile_level_hits();
        self.handle_projectile_enemy_hits();
        self.handle_projectile_player_hits();
        self.projectiles.retain(|projectile| projectile.alive);
    }

    fn handle_projectile_level_hits(&mut self) {
        for projectile in &mut self.projectiles {
            if !projectile.alive
                || projectile.piercing
                || projectile.kind == ProjectileKind::Shockwave
            {
                continue;
            }
            for platform in &self.level.platforms {
                if projectile.rect().overlaps(&platform.rect) {
                    projectile.alive = false;
                    break;
                }
            }
        }
    }

    fn handle_projectile_enemy_hits(&mut self) {
        let mut boss_explosion = false;
        let mut enemy_hit = false;
        let mut boss_hit = false;
        for projectile in &mut self.projectiles {
            if !projectile.alive || projectile.owner != ProjectileOwner::Player {
                continue;
            }

            for enemy in &mut self.enemies {
                if !enemy.alive || !projectile.rect().overlaps(&enemy.rect()) {
                    continue;
                }
                let killed = enemy.hit(projectile.damage);
                enemy_hit = true;
                self.particles.extend(Particle::burst(
                    enemy.center(),
                    color_u8!(255, 213, 94, 255),
                    if killed { 14 } else { 5 },
                    if killed { 190.0 } else { 90.0 },
                ));
                if killed {
                    self.score += match enemy.kind {
                        EnemyKind::BugCrawler => 120,
                        EnemyKind::ExceptionBat => 160,
                        EnemyKind::BuildTurret => 260,
                        EnemyKind::TestBot => 190,
                    };
                    if let Some(kind) = enemy.drop.take() {
                        self.pickups
                            .push(Pickup::new(kind, enemy.center().x, enemy.center().y));
                    }
                }
                if !projectile.piercing {
                    projectile.alive = false;
                    break;
                }
            }

            if self.boss.active
                && !self.boss.defeated
                && projectile.alive
                && projectile.rect().overlaps(&self.boss.rect())
            {
                let defeated = self.boss.hit(projectile.damage, &mut self.particles);
                boss_hit = true;
                if defeated {
                    boss_explosion = true;
                }
                projectile.alive = false;
            }
        }

        self.enemies.retain(|enemy| enemy.alive);
        if boss_explosion {
            self.audio.play(Sfx::Boss);
            self.shake(0.7, 22.0);
        } else if boss_hit || enemy_hit {
            self.audio.play(Sfx::Hit);
        }
    }

    fn handle_projectile_player_hits(&mut self) {
        let mut took_damage = false;
        let mut took_life_hit = false;
        for projectile in &mut self.projectiles {
            if !projectile.alive || projectile.owner == ProjectileOwner::Player {
                continue;
            }
            if projectile.rect().overlaps(&self.player.rect()) {
                projectile.alive = false;
                let before_health = self.player.health;
                let before_lives = self.player.lives;
                let life_lost = self.player.take_damage(projectile.damage, self.checkpoint);
                took_damage |= life_lost
                    || self.player.health != before_health
                    || self.player.lives != before_lives;
                took_life_hit |= life_lost;
            }
        }
        if took_damage {
            self.player_damage_feedback(took_life_hit, "Conflict hit");
        }
    }

    fn update_pickups(&mut self, dt: f32) {
        let mut collected_notices = Vec::new();
        for pickup in &mut self.pickups {
            pickup.update(dt, &self.level);
            if pickup.rect().overlaps(&self.player.rect()) {
                pickup.collected = true;
                let kind = pickup.kind;
                match pickup.kind {
                    PickupKind::Weapon(weapon) => self.player.weapon = weapon,
                    PickupKind::Health => self.player.heal(3),
                    PickupKind::Shield => self.player.shield(6.0),
                }
                collected_notices.push(match kind {
                    PickupKind::Weapon(weapon) => {
                        (format!("{} equipped", weapon.name()), weapon_color(weapon))
                    }
                    PickupKind::Health => {
                        ("Health restored".to_string(), color_u8!(119, 255, 150, 255))
                    }
                    PickupKind::Shield => (
                        "Test Shield online".to_string(),
                        color_u8!(147, 176, 255, 255),
                    ),
                });
                self.score += 75;
                self.particles.extend(Particle::burst(
                    pickup.pos,
                    color_u8!(106, 231, 255, 255),
                    9,
                    120.0,
                ));
            }
        }
        self.pickups.retain(|pickup| !pickup.collected);
        for (text, color) in collected_notices {
            self.audio.play(Sfx::Pickup);
            self.push_notice(text, color, 2.4);
        }
    }

    fn update_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.update(dt);
        }
        self.particles.retain(Particle::alive);
    }

    fn update_notices(&mut self, dt: f32) {
        for notice in &mut self.notices {
            notice.timer -= dt;
        }
        self.notices.retain(|notice| notice.timer > 0.0);
    }

    fn handle_player_hazards(&mut self) {
        let player_rect = self.player.rect();
        let hazard_hit = self
            .level
            .hazards
            .iter()
            .enumerate()
            .any(|(index, hazard)| {
                self.level.hazard_active(index, self.elapsed) && hazard.rect.overlaps(&player_rect)
            });

        if hazard_hit {
            let before_health = self.player.health;
            let before_lives = self.player.lives;
            let life_lost = self.player.take_damage(1, self.checkpoint);
            if life_lost || self.player.health != before_health || self.player.lives != before_lives
            {
                self.player_damage_feedback(life_lost, "Hazard hit");
            }
        }
    }

    fn player_damage_feedback(&mut self, life_lost: bool, source: &str) {
        self.damage_flash_timer = 0.22;
        self.audio.play(Sfx::Hit);
        self.shake(
            if life_lost { 0.28 } else { 0.16 },
            if life_lost { 11.0 } else { 6.0 },
        );
        self.particles.extend(Particle::burst(
            self.player.center(),
            color_u8!(255, 103, 97, 255),
            if life_lost { 18 } else { 8 },
            if life_lost { 190.0 } else { 110.0 },
        ));

        if life_lost {
            if self.player.lives > 0 {
                self.push_notice(
                    format!("Respawned at checkpoint ({})", source),
                    color_u8!(255, 213, 94, 255),
                    2.5,
                );
            } else {
                self.push_notice("Patch integrity failed", color_u8!(255, 103, 97, 255), 2.5);
            }
        } else {
            self.push_notice("Patch integrity hit", color_u8!(255, 103, 97, 255), 1.6);
        }
    }

    fn reset_run(&mut self) {
        self.level = Level::new();
        self.checkpoint = vec2(90.0, 480.0);
        self.checkpoint_stage = 0;
        self.encounter_stage = 0;
        self.player.reset(self.checkpoint);
        self.enemies = starting_enemies();
        self.pickups = starting_pickups();
        self.projectiles.clear();
        self.particles.clear();
        self.notices.clear();
        self.boss = Boss::new();
        self.camera_x = 0.0;
        self.score = 0;
        self.elapsed = 0.0;
        self.shake_timer = 0.0;
        self.shake_strength = 0.0;
        self.damage_flash_timer = 0.0;
        self.mouse_seen = false;
        self.mouse_aim_active = false;
        self.mouse_tip_shown = false;
        self.push_notice("Patch Force deployed", color_u8!(106, 231, 255, 255), 2.0);
    }

    fn with_run_state(mut self) -> Self {
        self.enemies = starting_enemies();
        self.pickups = starting_pickups();
        self
    }

    fn shake(&mut self, seconds: f32, strength: f32) {
        self.shake_timer = self.shake_timer.max(seconds);
        self.shake_strength = self.shake_strength.max(strength);
    }

    fn push_notice(&mut self, text: impl Into<String>, color: Color, timer: f32) {
        self.notices.push(Notice {
            text: text.into(),
            timer,
            color,
        });
        if self.notices.len() > 3 {
            self.notices.remove(0);
        }
    }
}

fn draw_line_points(start: Vec2, end: Vec2, thickness: f32, color: Color) {
    draw_line(start.x, start.y, end.x, end.y, thickness, color);
}

fn virtual_screen_camera() -> Camera2D {
    Camera2D {
        target: vec2(SCREEN_W * 0.5, SCREEN_H * 0.5),
        zoom: vec2(2.0 / SCREEN_W, 2.0 / SCREEN_H),
        ..Default::default()
    }
}

fn weapon_color(weapon: Weapon) -> Color {
    match weapon {
        Weapon::PatchRifle => color_u8!(106, 231, 255, 255),
        Weapon::SpreadDiff => color_u8!(255, 213, 94, 255),
        Weapon::RefactorBeam => color_u8!(176, 255, 155, 255),
        Weapon::HotfixSmg => color_u8!(255, 120, 167, 255),
    }
}

fn starting_enemies() -> Vec<Enemy> {
    vec![
        Enemy::new(EnemyKind::BugCrawler, 520.0, 456.0, None),
        Enemy::new(EnemyKind::ExceptionBat, 850.0, 250.0, None),
        Enemy::new(
            EnemyKind::BuildTurret,
            1110.0,
            341.0,
            Some(PickupKind::Weapon(Weapon::SpreadDiff)),
        ),
        Enemy::new(EnemyKind::BugCrawler, 1570.0, 456.0, None),
        Enemy::new(EnemyKind::ExceptionBat, 1840.0, 295.0, None),
        Enemy::new(EnemyKind::TestBot, 2290.0, 398.0, None),
        Enemy::new(
            EnemyKind::BuildTurret,
            2560.0,
            294.0,
            Some(PickupKind::Weapon(Weapon::HotfixSmg)),
        ),
        Enemy::new(EnemyKind::TestBot, 3090.0, 398.0, None),
        Enemy::new(EnemyKind::ExceptionBat, 3440.0, 250.0, None),
        Enemy::new(
            EnemyKind::BuildTurret,
            3970.0,
            274.0,
            Some(PickupKind::Weapon(Weapon::RefactorBeam)),
        ),
        Enemy::new(EnemyKind::TestBot, 4510.0, 446.0, None),
        Enemy::new(EnemyKind::BugCrawler, 4750.0, 456.0, None),
    ]
}

fn starting_pickups() -> Vec<Pickup> {
    vec![
        Pickup::new(PickupKind::Weapon(Weapon::SpreadDiff), 745.0, 285.0),
        Pickup::new(PickupKind::Health, 1845.0, 360.0),
        Pickup::new(PickupKind::Shield, 2660.0, 292.0),
        Pickup::new(PickupKind::Weapon(Weapon::HotfixSmg), 3220.0, 386.0),
        Pickup::new(PickupKind::Health, 3730.0, 350.0),
        Pickup::new(PickupKind::Weapon(Weapon::RefactorBeam), 4040.0, 274.0),
    ]
}

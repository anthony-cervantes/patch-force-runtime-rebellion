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
    checkpoint: Vec2,
    mouse_seen: bool,
    mouse_aim_active: bool,
    last_mouse_screen: Vec2,
}

impl Game {
    pub fn new(renderer: SpriteRenderer) -> Self {
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
            checkpoint,
            mouse_seen: false,
            mouse_aim_active: false,
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
        if self.shake_timer == 0.0 {
            self.shake_strength = 0.0;
        }

        self.player.update(dt, &self.level);
        self.update_player_mouse_facing();
        if self.player.pos.x > 2320.0 {
            self.checkpoint = vec2(2260.0, 480.0);
        }
        if self.player.pos.x > self.level.boss_start + 120.0 {
            self.checkpoint = vec2(self.level.boss_start + 80.0, 480.0);
        }

        if self.player.pos.x > self.level.boss_start {
            self.boss.active = true;
            self.checkpoint = vec2(self.level.boss_start + 80.0, 480.0);
            self.player.pos.x = self.player.pos.x.max(self.level.boss_start + 34.0);
        }

        self.handle_shooting();
        self.update_enemies(dt);
        self.update_boss(dt);
        self.update_projectiles(dt);
        self.update_pickups(dt);
        self.update_particles(dt);
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
        self.particles.push(Particle::new(
            muzzle,
            aim * 70.0,
            0.12,
            6.0,
            color_u8!(255, 245, 212, 255),
        ));
    }

    fn update_mouse_aim_state(&mut self) {
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

    fn virtual_mouse_position(&self) -> Vec2 {
        let (mx, my) = mouse_position();
        let scale_x = SCREEN_W / screen_width().max(1.0);
        let scale_y = SCREEN_H / screen_height().max(1.0);
        vec2(mx * scale_x, my * scale_y)
    }

    fn update_enemies(&mut self, dt: f32) {
        let player_pos = self.player.center();
        let mut spawned_shots = Vec::new();
        let mut took_life_hit = false;
        for enemy in &mut self.enemies {
            if enemy.rect().x > self.camera_x + SCREEN_W + 260.0
                || enemy.rect().x + enemy.rect().w < self.camera_x - 360.0
            {
                continue;
            }
            enemy.update(dt, player_pos, &self.level, &mut spawned_shots);
            if enemy.alive && enemy.rect().overlaps(&self.player.rect()) {
                took_life_hit |= self.player.take_damage(1, self.checkpoint);
            }
        }
        self.projectiles.extend(spawned_shots);
        if took_life_hit {
            self.shake(0.18, 6.0);
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
        for projectile in &mut self.projectiles {
            if !projectile.alive || projectile.owner != ProjectileOwner::Player {
                continue;
            }

            for enemy in &mut self.enemies {
                if !enemy.alive || !projectile.rect().overlaps(&enemy.rect()) {
                    continue;
                }
                let killed = enemy.hit(projectile.damage);
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
                if defeated {
                    boss_explosion = true;
                }
                projectile.alive = false;
            }
        }

        self.enemies.retain(|enemy| enemy.alive);
        if boss_explosion {
            self.shake(0.7, 22.0);
        }
    }

    fn handle_projectile_player_hits(&mut self) {
        let mut took_life_hit = false;
        for projectile in &mut self.projectiles {
            if !projectile.alive || projectile.owner == ProjectileOwner::Player {
                continue;
            }
            if projectile.rect().overlaps(&self.player.rect()) {
                projectile.alive = false;
                took_life_hit |= self.player.take_damage(projectile.damage, self.checkpoint);
            }
        }
        if took_life_hit {
            self.shake(0.22, 8.0);
        }
    }

    fn update_pickups(&mut self, dt: f32) {
        for pickup in &mut self.pickups {
            pickup.update(dt, &self.level);
            if pickup.rect().overlaps(&self.player.rect()) {
                pickup.collected = true;
                match pickup.kind {
                    PickupKind::Weapon(weapon) => self.player.weapon = weapon,
                    PickupKind::Health => self.player.heal(3),
                    PickupKind::Shield => self.player.shield(6.0),
                }
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
    }

    fn update_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.update(dt);
        }
        self.particles.retain(Particle::alive);
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

        if hazard_hit && self.player.take_damage(1, self.checkpoint) {
            self.shake(0.18, 8.0);
        }
    }

    fn reset_run(&mut self) {
        self.level = Level::new();
        self.checkpoint = vec2(90.0, 480.0);
        self.player.reset(self.checkpoint);
        self.enemies = starting_enemies();
        self.pickups = starting_pickups();
        self.projectiles.clear();
        self.particles.clear();
        self.boss = Boss::new();
        self.camera_x = 0.0;
        self.score = 0;
        self.elapsed = 0.0;
        self.shake_timer = 0.0;
        self.shake_strength = 0.0;
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

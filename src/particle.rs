use macroquad::prelude::*;

pub struct Particle {
    pub pos: Vec2,
    vel: Vec2,
    life: f32,
    max_life: f32,
    size: f32,
    color: Color,
}

impl Particle {
    pub fn new(pos: Vec2, vel: Vec2, life: f32, size: f32, color: Color) -> Self {
        Self {
            pos,
            vel,
            life,
            max_life: life,
            size,
            color,
        }
    }

    pub fn burst(pos: Vec2, color: Color, count: usize, speed: f32) -> Vec<Self> {
        let mut particles = Vec::with_capacity(count);
        for i in 0..count {
            let angle = i as f32 / count.max(1) as f32 * std::f32::consts::TAU;
            let wobble = ((i * 37) % 19) as f32 / 19.0;
            let vel = vec2(angle.cos(), angle.sin()) * (speed * (0.55 + wobble));
            particles.push(Self::new(
                pos,
                vel,
                0.35 + wobble * 0.25,
                4.0 + wobble * 4.0,
                color,
            ));
        }
        particles
    }

    pub fn update(&mut self, dt: f32) {
        self.life -= dt;
        self.vel.y += 460.0 * dt;
        self.pos += self.vel * dt;
    }

    pub fn alive(&self) -> bool {
        self.life > 0.0
    }

    pub fn draw(&self, camera_x: f32, y_offset: f32) {
        let alpha = (self.life / self.max_life).clamp(0.0, 1.0);
        let mut color = self.color;
        color.a *= alpha;
        draw_rectangle(
            self.pos.x - camera_x - self.size * 0.5,
            self.pos.y + y_offset - self.size * 0.5,
            self.size,
            self.size,
            color,
        );
    }
}

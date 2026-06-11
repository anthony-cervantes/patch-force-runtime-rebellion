use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};

#[derive(Clone, Copy)]
pub enum Sfx {
    Shoot,
    Pickup,
    Hit,
    Checkpoint,
    Boss,
    Victory,
}

pub struct AudioBank {
    shoot: Option<Sound>,
    pickup: Option<Sound>,
    hit: Option<Sound>,
    checkpoint: Option<Sound>,
    boss: Option<Sound>,
    victory: Option<Sound>,
}

impl AudioBank {
    pub async fn load() -> Self {
        Self {
            shoot: load_sound("assets/sfx/shoot.wav").await.ok(),
            pickup: load_sound("assets/sfx/pickup.wav").await.ok(),
            hit: load_sound("assets/sfx/hit.wav").await.ok(),
            checkpoint: load_sound("assets/sfx/checkpoint.wav").await.ok(),
            boss: load_sound("assets/sfx/boss.wav").await.ok(),
            victory: load_sound("assets/sfx/victory.wav").await.ok(),
        }
    }

    pub fn play(&self, sfx: Sfx) {
        let (sound, volume) = match sfx {
            Sfx::Shoot => (self.shoot.as_ref(), 0.22),
            Sfx::Pickup => (self.pickup.as_ref(), 0.45),
            Sfx::Hit => (self.hit.as_ref(), 0.5),
            Sfx::Checkpoint => (self.checkpoint.as_ref(), 0.5),
            Sfx::Boss => (self.boss.as_ref(), 0.55),
            Sfx::Victory => (self.victory.as_ref(), 0.58),
        };

        if let Some(sound) = sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume,
                },
            );
        }
    }
}

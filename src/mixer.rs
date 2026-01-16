use std::os;

use crate::traits::AudioSource;

pub struct Mixer {
    sources: Vec<Box<dyn AudioSource>>,
    level: f32,
}

impl Mixer {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            level: 0.1,
        }
    }

    pub fn add(&mut self, source: Box<dyn AudioSource>) {
        self.sources.push(source);
    }

    pub fn set_level(&mut self, level: f32) {
        self.level = level;
    }
    pub fn get_level(&self) -> f32 {
        self.level
    }
}

impl AudioSource for Mixer {
    fn next_sample(&mut self) -> f32 {
        let mut sum = 0.0;
        for source in &mut self.sources {
            sum += source.next_sample();
        }

        let mixed = sum * self.level;

        mixed.clamp(-1.0, 1.0)
    }
}

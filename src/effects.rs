use crate::traits::AudioSource;

pub struct GainEffect {
    input: Box<dyn AudioSource>,
    volume: f32,
}

impl GainEffect {
    pub fn new(input: Box<dyn AudioSource>, volume: f32) -> Self {
        Self { input, volume }
    }
}

impl AudioSource for GainEffect {
    fn next_sample(&mut self) -> f32 {
        self.input.next_sample() * self.volume
    }
}

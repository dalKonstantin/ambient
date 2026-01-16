use crate::traits::AudioSource;
use std::f32::consts::PI;

pub struct SineOsc {
    phase: f32,
    frequency: f32,
    sample_rate: f32,
}

impl SineOsc {
    pub fn new(freq: f32, sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            frequency: freq,
            sample_rate,
        }
    }
}

impl AudioSource for SineOsc {
    fn next_sample(&mut self) -> f32 {
        let value = (self.phase * 2.0 * PI).sin();
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;

        value
    }
}

pub struct SawOsc {
    phase: f32,
    frequency: f32,
    sample_rate: f32,
}

impl SawOsc {
    pub fn new(freq: f32, sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            frequency: freq,
            sample_rate,
        }
    }
}

impl AudioSource for SawOsc {
    fn next_sample(&mut self) -> f32 {
        let value = (self.phase * 2.0) - 1.0;
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;

        value
    }
}

pub struct SquareOsc {
    phase: f32,
    frequency: f32,
    sample_rate: f32,
}

impl SquareOsc {
    pub fn new(freq: f32, sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            frequency: freq,
            sample_rate,
        }
    }
}

impl AudioSource for SquareOsc {
    fn next_sample(&mut self) -> f32 {
        let value = if self.phase < 0.5 { 1.0 } else { -1.0 };
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;

        value
    }
}

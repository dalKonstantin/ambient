use crate::{parameter::Parameter, traits::AudioSource};
use std::f32::consts::PI;

pub struct SineOsc {
    phase: f32,
    frequency: Parameter,
    sample_rate: f32,
}

impl SineOsc {
    pub fn new(freq: Parameter, sample_rate: f32) -> Self {
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
        let curr_freq = self.frequency.get();
        self.phase = (self.phase + curr_freq / self.sample_rate) % 1.0;

        value
    }
}

pub struct SawOsc {
    phase: f32,
    frequency: Parameter,
    sample_rate: f32,
}

impl SawOsc {
    pub fn new(freq: Parameter, sample_rate: f32) -> Self {
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
        let curr_freq = self.frequency.get();
        self.phase = (self.phase + curr_freq / self.sample_rate) % 1.0;

        value
    }
}

pub struct SquareOsc {
    phase: f32,
    frequency: Parameter,
    sample_rate: f32,
}

impl SquareOsc {
    pub fn new(freq: Parameter, sample_rate: f32) -> Self {
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
        let curr_freq = self.frequency.get();
        self.phase = (self.phase + curr_freq / self.sample_rate) % 1.0;

        value
    }
}

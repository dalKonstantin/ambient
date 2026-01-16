// src/main.rs
mod effects;
mod engine;
mod generators;
mod mixer;
mod parameter;
mod traits;

use crate::effects::GainEffect;
use crate::engine::AudioEngine;
use crate::generators::{SawOsc, SineOsc, SquareOsc};
use crate::parameter::Parameter;
use rand::Rng;
use std::sync::mpsc::{Receiver, Sender, channel};

fn main() {
    let frequencies = [
        440.0, 493.8833, 523.2511, 587.3295, 659.2551, 698.4565, 783.9909,
    ];
    let mut rng = rand::thread_rng();
    println!("initializing engine");
    let player = AudioEngine::new();
    let sr = player.sample_rate;

    let global_volume = 0.1;
    player.set_global_volume(global_volume);

    let freq_par1 = Parameter::new(frequencies[0]);
    let freq_par2 = Parameter::new(frequencies[2]);
    let freq_par3 = Parameter::new(110.0);

    println!("playing...");
    player.play(Box::new(SineOsc::new(freq_par1.clone(), sr)));
    player.play(Box::new(SineOsc::new(freq_par2.clone(), sr)));
    player.play(Box::new(SquareOsc::new(freq_par3.clone(), sr)));

    loop {
        let rnd = rng.gen_range(0..6);
        let freq = frequencies[rnd];
        let freq2 = frequencies[(rnd + 2) / (frequencies.len() - 1)];
        freq_par1.set(freq);
        freq_par2.set(freq2);
        std::thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..500)));
    }
}

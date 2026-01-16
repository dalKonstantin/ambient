// src/main.rs
mod effects;
mod engine;
mod generators;
mod mixer;
mod traits;

use crate::effects::GainEffect;
use crate::engine::AudioEngine;
use crate::generators::{SawOsc, SineOsc, SquareOsc};

fn main() {
    println!("initializing engine");
    let player = AudioEngine::new();
    let sr = player.sample_rate;

    let global_volume = 0.1;
    player.set_global_volume(global_volume);

    println!("Playing chord");

    let sin1 = Box::new(SawOsc::new(110.0, sr));
    let sin1_proc = Box::new(GainEffect::new(sin1, 0.7));
    player.play(sin1_proc);
    // player.play(Box::new(SineOsc::new(440.0, sr)));
    // player.play(Box::new(SineOsc::new(220.0, sr)));

    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("Fade out");
    for i in (0..100).rev() {
        let vol = i as f32 / 100.0;
        player.set_global_volume(vol * global_volume);
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    std::thread::park();
}

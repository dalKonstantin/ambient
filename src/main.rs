mod effects;
mod engine;
mod generators;
mod mixer;
mod parameter;
mod traits;

use crate::effects::GainEffect;
use crate::engine::AudioEngine;
use crate::generators::{NoiseOsc, SineOsc};
use crate::parameter::Parameter;
use rand::Rng;

fn main() {
    let frequencies = [
        440.0, 493.8833, 523.2511, 587.3295, 659.2551, 698.4565, 783.9909,
    ];
    let mut rng = rand::rng();
    println!("initializing engine");
    let player = AudioEngine::new();
    let sr = player.sample_rate;

    let global_volume = 0.1;
    player.set_global_volume(global_volume);

    let freq_par = [
        Parameter::new(frequencies[2]),
        Parameter::new(110.0),
        Parameter::new(0.01),
    ];

    println!("playing...");
    player.play(Box::new(SineOsc::new(freq_par[0].clone(), sr)));
    player.play(Box::new(SineOsc::new(freq_par[1].clone(), sr)));
    player.play(Box::new(SineOsc::new(freq_par[2].clone(), sr)));

    let noise = Box::new(NoiseOsc::new(1));
    let noise_amp = Box::new(GainEffect::new(noise, 0.1));
    player.play(noise_amp);
    loop {
        let rnd = rng.random_range(0..6);
        let freq = [
            frequencies[rnd],
            frequencies[(rnd + 2) / (frequencies.len() - 1)],
        ];
        freq_par[0].set(freq[0]);
        freq_par[1].set(freq[1]);
        std::thread::sleep(std::time::Duration::from_millis(rng.random_range(300..500)));
    }
}

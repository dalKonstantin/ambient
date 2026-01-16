use std::sync::mpsc::{Receiver, Sender, channel};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::{mixer::Mixer, traits::AudioSource};

enum EngineMessage {
    AddSource(Box<dyn AudioSource>),
    SetGlobalVolume(f32),
}

pub struct AudioEngine {
    _stream: cpal::Stream,
    command_sender: Sender<EngineMessage>,
    pub sample_rate: f32,
}

impl AudioEngine {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No device found");
        let config = device.default_output_config().expect("No config");
        let sample_rate = config.sample_rate() as f32;

        let (sender, receiver) = channel::<EngineMessage>();

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), receiver),
            cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), receiver),
            cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), receiver),
            _ => panic!("Unsupported format"),
        };

        stream.play().expect("Failed to play");

        Self {
            _stream: stream,
            command_sender: sender,
            sample_rate,
        }
    }

    pub fn play(&self, source: Box<dyn AudioSource>) {
        self.command_sender
            .send(EngineMessage::AddSource(source))
            .expect("AudioEngine is dead");
    }

    pub fn set_global_volume(&self, volume: f32) {
        self.command_sender
            .send(EngineMessage::SetGlobalVolume(volume))
            .expect("AudioEngine is dead");
    }
}

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    receiver: Receiver<EngineMessage>,
) -> cpal::Stream
where
    T: cpal::Sample + cpal::SizedSample + cpal::FromSample<f32>,
{
    let channels = config.channels as usize;

    let mut mixer = Mixer::new();

    device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                while let Ok(msg) = receiver.try_recv() {
                    match msg {
                        EngineMessage::AddSource(source) => {
                            mixer.add(source);
                        }
                        EngineMessage::SetGlobalVolume(vol) => {
                            mixer.set_level(vol);
                        }
                    }
                }

                for frame in data.chunks_mut(channels) {
                    let value_f32 = mixer.next_sample();
                    let value = T::from_sample(value_f32);
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )
        .unwrap()
}

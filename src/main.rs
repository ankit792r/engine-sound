mod cylinder;
mod engine;
mod inputs;
mod oscillator;

use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

use crate::{engine::engine::Engine, inputs::keyboard::spawn_throttle_input};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device");

    let config = device.default_output_config().expect("No default config");

    let sample_rate = config.sample_rate() as f32;

    let throttle_target = Arc::new(AtomicU32::new(0));

    // spawn_throttle_input(throttle_target.clone());

    let mut eng = Engine::new(sample_rate);

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                // let throttle = throttle_target.load(Ordering::Relaxed) as f32 / 1000.0;

                // eng.set_throttle(throttle);

                for sample in data.iter_mut() {
                    *sample = eng.next_sample() * 3.0;
                }
            },
            move |err| {
                eprintln!("Stream error: {}", err);
            },
            None,
        )
        .unwrap();

    stream.play().unwrap();

    println!("playing 440Hz tone");
    std::thread::sleep(std::time::Duration::from_secs(10));
}

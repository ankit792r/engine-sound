mod cylinder;
mod engine;
mod oscillator;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::engine::Engine;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device");

    let config = device.default_output_config().expect("No default config");

    let sample_rate = config.sample_rate() as f32;

    let mut eng = Engine::new(sample_rate);
    eng.set_rpm(200.0);

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
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

mod oscillator;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use oscillator::oscillator_v1::Oscilltor;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();

    let config = device.default_output_config().unwrap();

    let sample_rate = config.sample_rate() as f32;

    let mut osc = Oscilltor {
        phase: 0.0,
        frequency: 440.0,
        sample_rate,
    };

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                for sample in data.iter_mut() {
                    *sample = osc.next_sample() * 0.1;
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
    std::thread::sleep(std::time::Duration::from_secs(1000));
}

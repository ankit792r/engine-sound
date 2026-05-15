mod oscillator;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use oscillator::oscillator_v1::Oscilltor;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device");

    let config = device.default_output_config().expect("No default config");

    let sample_rate = config.sample_rate() as f32;

    let mut oscillators = vec![
        Oscilltor::new(110.0, sample_rate),
        Oscilltor::new(220.0, sample_rate),
        Oscilltor::new(330.0, sample_rate),
    ];

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                for sample in data.iter_mut() {
                    let mut mixed = 0.0;

                    for osc in oscillators.iter_mut() {
                        mixed += osc.next_sample() * 0.2;
                    }

                    *sample = mixed;
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

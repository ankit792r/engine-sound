mod oscillator;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use oscillator::oscillator_v1::{Oscillator, Waveform};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device");

    let config = device.default_output_config().expect("No default config");

    let sample_rate = config.sample_rate() as f32;

    let mut oscillators = vec![
        Oscillator::new(150.0, sample_rate, Waveform::Sine),
        Oscillator::new(70.0, sample_rate, Waveform::Saw),
        Oscillator::new(60.0, sample_rate, Waveform::Square),
        Oscillator::new(120.0, sample_rate, Waveform::Triangle),
        Oscillator::new(80.0, sample_rate, Waveform::Noise),
    ];

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                for sample in data.iter_mut() {
                    let mut mixed = 0.0;

                    for osc in oscillators.iter_mut() {
                        mixed += osc.next_sample() * 0.4;
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
    std::thread::sleep(std::time::Duration::from_secs(5));
}

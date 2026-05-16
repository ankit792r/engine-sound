use std::f32::consts::PI;

use rand::RngExt;

pub enum Waveform {
    Sine,
    Saw,
    Square,
    Noise,
    Triangle,
}

pub struct Oscillator {
    pub phase: f32,
    pub frequency: f32,
    pub sample_rate: f32,
    pub waveform: Waveform,
}

impl Oscillator {
    pub fn new(frequency: f32, sample_rate: f32, waveform: Waveform) -> Self {
        Self {
            phase: 0.0,
            frequency,
            sample_rate,
            waveform,
        }
    }

    fn generate_noise(&self) -> f32 {
        let mut rng = rand::rng();
        rng.random_range(-1.0..1.0)
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = match self.waveform {
            Waveform::Sine => (2.0 * PI * self.phase).sin(),
            Waveform::Saw => 2.0 * self.phase - 1.0,
            Waveform::Square => {
                if self.phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Triangle => 2.0 * (16.0 * self.phase - 1.0).abs() -1.0,
            Waveform::Noise => self.generate_noise(),
        };
        self.phase += self.frequency / self.sample_rate * 0.08;

        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }
}

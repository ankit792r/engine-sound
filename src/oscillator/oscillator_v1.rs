use std::f32::consts::PI;

use rand::{RngExt, rngs::ThreadRng};

pub enum Waveform {
    Sine,
    Saw,
    Square,
    Noice,
    Triangle,
}

pub struct Oscilltor {
    pub phase: f32,
    pub frequency: f32,
    pub sample_rate: f32,
    pub waveform: Waveform,
}

impl Oscilltor {
    pub fn new(frequency: f32, sample_rate: f32, waveform: Waveform) -> Self {
        Self {
            phase: 0.0,
            frequency,
            sample_rate,
            waveform,
        }
    }

    fn generate_noice(&self) -> f32 {
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
            Waveform::Triangle => 2.0 * (2.0 * self.phase - 1.0).abs() - 1.0,
            Waveform::Noice => self.generate_noice(),
        };
        self.phase += self.frequency / self.sample_rate;

        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }
}

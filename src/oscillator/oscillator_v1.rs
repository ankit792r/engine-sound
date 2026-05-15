use std::f32::consts::PI;

pub struct Oscilltor {
    pub phase: f32,
    pub frequency: f32,
    pub sample_rate: f32,
}

impl Oscilltor {
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            frequency,
            sample_rate,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = (2.0 * PI * self.phase).sin();
        self.phase += self.frequency / self.sample_rate;

        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }
}

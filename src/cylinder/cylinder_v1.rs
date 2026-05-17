use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub enum Waveform {
    Sine,
    Saw,
    Triangle,
    Square,
    Noise,
}

pub struct Cylinder {
    /// current phase 0.0 -> 1.0
    phase: f32,

    /// where this cylinder fires in the engine cycle
    firing_offset: f32,

    /// audio sample rate
    sample_rate: f32,

    /// 2 or 4 stroke
    strokes: u8,

    /// waveform character
    waveform: Waveform,
}

impl Cylinder {
    pub fn new(sample_rate: f32, strokes: u8, firing_offset: f32, waveform: Waveform) -> Self {
        Self {
            phase: 0.0,
            firing_offset,
            sample_rate,
            strokes,
            waveform,
        }
    }

    fn combustion_frequency(&self, rpm: f32) -> f32 {
        match self.strokes {
            // fires every revolution
            2 => rpm / 60.0,

            // fires every 2 revolutions
            4 => rpm / 120.0,

            _ => rpm / 120.0,
        }
    }

    fn waveform_sample(&self, t: f32) -> f32 {
        match self.waveform {
            Waveform::Sine => (2.0 * PI * t).sin(),

            Waveform::Saw => 2.0 * t - 1.0,

            Waveform::Square => {
                if t < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }

            Waveform::Triangle => 2.0 * (2.0 * t - 1.0).abs() - 1.0,

            Waveform::Noise => rand::random::<f32>() * 2.0 - 1.0,
        }
    }

    fn combustion_envelope(&self, t: f32) -> f32 {
        if t < 0.1 { 1.0 - (t * 10.0) } else { 0.0 }
    }

    pub fn next_sample(&mut self, rpm: f32) -> f32 {
        let fire_freq = self.combustion_frequency(rpm);
        self.phase += fire_freq / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        let local_phase = (self.phase + self.firing_offset) % 1.0;
        let envelope = self.combustion_envelope(local_phase);
        let wave = self.waveform_sample(local_phase);
        wave * envelope
    }
}

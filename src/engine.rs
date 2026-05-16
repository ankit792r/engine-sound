use crate::cylinder::cylinder_v1::{Cylinder, Waveform};

pub struct Engine {
    cylinders: Vec<Cylinder>,
}

impl Engine {
    pub fn v8(rpm: f32, sample_rate: f32) -> Self {
        let cylinders = vec![
            Cylinder::new(rpm, sample_rate, 4, 0.0, Waveform::Saw),
            Cylinder::new(rpm, sample_rate, 4, 0.125, Waveform::Saw),
            Cylinder::new(rpm, sample_rate, 4, 0.25, Waveform::Saw),
            Cylinder::new(rpm, sample_rate, 4, 0.375, Waveform::Saw),
            Cylinder::new(rpm, sample_rate, 4, 0.5, Waveform::Saw),
            Cylinder::new(rpm, sample_rate, 4, 0.625, Waveform::Saw),
            Cylinder::new(rpm, sample_rate, 4, 0.75, Waveform::Saw),
            Cylinder::new(rpm, sample_rate, 4, 0.875, Waveform::Saw),
        ];

        Self { cylinders }
    }

    pub fn inline4(rpm: f32, sample_rate: f32) -> Self {
        let cylinders = vec![
            Cylinder::new(rpm, sample_rate, 4, 0.0, Waveform::Saw),
            Cylinder::new(rpm, sample_rate, 4, 0.25, Waveform::Saw),
            Cylinder::new(rpm, sample_rate, 4, 0.5, Waveform::Saw),
            Cylinder::new(rpm, sample_rate, 4, 0.75, Waveform::Saw),
        ];

        Self { cylinders }
    }


    pub fn inline2(rpm: f32, sample_rate: f32) -> Self {
        let cylinders = vec![
            Cylinder::new(rpm, sample_rate, 4, 0.25, Waveform::Saw),
            Cylinder::new(rpm, sample_rate, 4, 0.75, Waveform::Saw),
        ];

        Self { cylinders }
    }


    pub fn single(rpm: f32, sample_rate: f32) -> Self {
        let cylinders = vec![
            Cylinder::new(rpm, sample_rate, 4, 1.0, Waveform::Saw),
        ];

        Self { cylinders }
    }

    pub fn next_sample(&mut self) -> f32 {
        let mut sample = 0.0;

        for cylinder in &mut self.cylinders {
            sample += cylinder.next_sample();
        }

        // normalize
        sample / self.cylinders.len() as f32
    }

    pub fn set_rpm(&mut self, rpm: f32) {
        for cylinder in &mut self.cylinders {
            cylinder.set_rpm(rpm);
        }
    }
}

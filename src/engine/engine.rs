use crate::{
    cylinder::cylinder_v1::{Cylinder, Waveform},
    engine::{
        rpm::RpmDynamics,
        throttle::{self, Throttle},
        torque::{self, TorqueCurve},
    },
};

pub enum EngineType {
    V8,
    Inline4,
    Inline2,
    Single,
}

pub struct Engine {
    cylinders: Vec<Cylinder>,
    current_rpm: f32,
    target_rpm: f32,
    rpm_velocity: f32,

    load: f32,
    idle_rpm: f32,
    max_rpm: f32,
    inertia: f32,

    throttle: Throttle,
    torque_curve: TorqueCurve,
    rpm_dynamics: RpmDynamics,
}

impl Engine {
    pub fn new(sample_rate: f32) -> Self {
        let cylinders = Self::make_cylinders(EngineType::Inline4, sample_rate);
        let throttle = Throttle::new();
        let rpm_dynamics = RpmDynamics::new();
        let torque_curve = TorqueCurve::new();
        Self {
            cylinders,
            current_rpm: 0.0,
            target_rpm: 0.0,
            rpm_velocity: 0.0,
            load: 0.0,
            idle_rpm: 0.0,
            max_rpm: 1500.0,
            inertia: 0.0,
            throttle,
            torque_curve,
            rpm_dynamics,
        }
    }

    pub fn update(&mut self) {
        self.throttle.update();

        let torque = self
            .torque_curve
            .get_torque(self.rpm_dynamics.rpm, self.throttle.value());

        self.rpm_dynamics.update(torque);
    }

    pub fn next_sample(&mut self) -> f32 {
        self.update();
        // smooth rpm transition
        // self.current_rpm += (self.target_rpm - self.current_rpm) * 0.001;

        self.current_rpm = self.rpm_dynamics.rpm;

        let mut sample = 0.0;

        for cylinder in &mut self.cylinders {
            sample += cylinder.next_sample(self.current_rpm);
        }

        // normalize: this will reduce the sound based on number of cylinders
        // sample / self.cylinders.len() as f32
        sample
    }

    pub fn set_rpm(&mut self, rpm: f32) {
        self.target_rpm = rpm;
    }
}

impl Engine {
    pub fn make_cylinders(engin_type: EngineType, sample_rate: f32) -> Vec<Cylinder> {
        match engin_type {
            EngineType::V8 => vec![
                Cylinder::new(sample_rate, 4, 0.0, Waveform::Saw),
                Cylinder::new(sample_rate, 4, 0.125, Waveform::Saw),
                Cylinder::new(sample_rate, 4, 0.25, Waveform::Saw),
                Cylinder::new(sample_rate, 4, 0.375, Waveform::Saw),
                Cylinder::new(sample_rate, 4, 0.5, Waveform::Saw),
                Cylinder::new(sample_rate, 4, 0.625, Waveform::Saw),
                Cylinder::new(sample_rate, 4, 0.75, Waveform::Saw),
                Cylinder::new(sample_rate, 4, 0.875, Waveform::Saw),
            ],
            EngineType::Inline4 => vec![
                Cylinder::new(sample_rate, 4, 0.0, Waveform::Saw),
                Cylinder::new(sample_rate, 4, 0.25, Waveform::Saw),
                Cylinder::new(sample_rate, 4, 0.5, Waveform::Saw),
                Cylinder::new(sample_rate, 4, 0.75, Waveform::Saw),
            ],
            EngineType::Inline2 => vec![
                Cylinder::new(sample_rate, 4, 0.0, Waveform::Saw),
                Cylinder::new(sample_rate, 4, 0.5, Waveform::Saw),
            ],
            EngineType::Single => vec![Cylinder::new(sample_rate, 4, 1.0, Waveform::Saw)],
        }
    }
}

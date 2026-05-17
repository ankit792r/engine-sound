pub struct TorqueCurve {
    idle_torque: f32,
    peak_torque: f32,
    max_rpm: f32,
}

impl TorqueCurve {
    pub fn new() -> Self {
        Self {
            idle_torque: 80.0,
            peak_torque: 400.0,
            max_rpm: 8000.0,
        }
    }

    pub fn get_torque(&self, rpm: f32, throttle: f32) -> f32 {
        let normalized_rpm = rpm / self.max_rpm;

        // simple fake torque curve
        let torque_shape = 1.0 - ((normalized_rpm - 0.5).abs() * 2.0);

        let torque = self.idle_torque + (self.peak_torque * torque_shape.max(0.0));

        torque * throttle
    }
}

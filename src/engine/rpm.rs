pub struct RpmDynamics {
    pub rpm: f32,
    rpm_velocity: f32,

    inertia: f32,
    friction: f32,

    idle_rpm: f32,
    max_rpm: f32,
}

impl RpmDynamics {
    pub fn new() -> Self {
        Self {
            rpm: 0.0,
            rpm_velocity: 0.0,

            inertia: 0.02,
            friction: 0.01,

            idle_rpm: 0.0,
            max_rpm: 8000.0,
        }
    }

    pub fn update(&mut self, torque: f32) {
        self.rpm_velocity += torque * self.inertia;

        self.rpm_velocity *= 1.0 - self.friction;

        self.rpm += self.rpm_velocity;

        self.rpm = self.rpm.clamp(self.idle_rpm, self.max_rpm);
    }
}

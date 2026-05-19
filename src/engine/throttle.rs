pub struct Throttle {
    value: f32,
    target: f32,
    response_speed: f32,

    rise_speed: f32,
    fall_speed: f32,
}

impl Throttle {
    pub fn new() -> Self {
        Self {
            value: 0.0,
            target: 0.0,
            response_speed: 0.01,

            rise_speed: 0.015,
            fall_speed: 0.01,
        }
    }

    pub fn update(&mut self) {
        let speed = if self.target > self.value {
            self.rise_speed
        } else {
            self.fall_speed
        };

        self.value += (self.target - self.value) * speed;
        // self.value += (self.target - self.value) * self.response_speed;
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target.clamp(0.0, 1.0);
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

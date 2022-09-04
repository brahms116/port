/* TODO this is a very simple implementation, should include accel and other simultenous rotating
 * elements */

pub struct RotationConfig {
    pub rotation_speed: f64,
}

impl RotationConfig {
    pub fn new(n: f64) -> Self {
        Self { rotation_speed: n }
    }
}

pub struct Rotation {
    pub config: RotationConfig,
    pub factor: f64,
}

impl Rotation {
    pub fn new(config: RotationConfig) -> Self {
        Self {
            config,
            factor: 0.0,
        }
    }

    pub fn is_left(&self) -> bool {
        self.factor < 0.0
    }
    pub fn is_right(&self) -> bool {
        self.factor > 0.0
    }
    pub fn stop(&mut self) {
        self.factor = 0.0;
    }
}

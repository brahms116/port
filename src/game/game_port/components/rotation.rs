/* TODO this is a very simple implementation, should include accel and other simultenous rotating
 * elements */

#[derive(PartialEq)]
pub enum RotationState {
    Idle,
    Left,
    Right,
}

type S = RotationState;

pub struct RotationConfig {
    pub rotation_speed: f64,
}

impl RotationConfig {
    pub fn new(n: f64) -> Self {
        Self { rotation_speed: n }
    }
}

pub struct Rotation {
    pub state: RotationState,
    pub config: RotationConfig,
    pub applied_vel: f64,
}

impl Rotation {
    pub fn new(config: RotationConfig) -> Self {
        Self {
            state: S::Idle,
            config,
            applied_vel: 0.0,
        }
    }

    pub fn is_left(&self) -> bool {
        self.state == S::Left
    }
    pub fn is_right(&self) -> bool {
        self.state == S::Right
    }

    pub fn left(&mut self) {
        self.state = S::Left;
        self.applied_vel = 0.0;
    }

    pub fn right(&mut self) {
        self.state = S::Right;
        self.applied_vel = 0.0;
    }

    pub fn stop(&mut self) {
        self.state = S::Idle;
    }
}

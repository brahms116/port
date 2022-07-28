/* TODO this is a very simple implementation, should include accel and other simultenous rotating
 * elements */

#[derive(PartialEq)]
pub enum RotationState {
    Idle,
    Left,
    Right,
}

type S = RotationState;

pub struct Rotation {
    pub state: RotationState,
}

impl Rotation {
    pub fn new() -> Self {
        Self { state: S::Idle }
    }

    pub fn is_left(&self) -> bool {
        self.state == S::Left
    }
    pub fn is_right(&self) -> bool {
        self.state == S::Right
    }

    pub fn left(&mut self) {
        self.state = S::Left
    }

    pub fn right(&mut self) {
        self.state = S::Right
    }

    pub fn stop(&mut self) {
        self.state = S::Idle
    }
}

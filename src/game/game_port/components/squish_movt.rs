use super::*;

#[derive(PartialEq)]
pub enum SquishMovementState {
    Idle,
    WaitingAnimation,
    StartedAnimation,
    ForceApplied,
    WaitingStopAnimation,
    StartedStopAnimation,
}

pub struct SquishMovement {
    pub state: SquishMovementState,
    // should add direction? maybe
}

impl SquishMovement {
    pub fn new() -> Self {
        Self {
            state: SquishMovementState::Idle,
        }
    }
    pub fn start_movt(&mut self) {
        self.state = SquishMovementState::WaitingAnimation
    }

    pub fn stop(&mut self) {
        self.state =
            SquishMovementState::WaitingStopAnimation
    }
}

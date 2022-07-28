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

type S = SquishMovementState;

pub struct SquishMovement {
    pub state: SquishMovementState,
    // should add direction? maybe
}

impl SquishMovement {
    pub fn new() -> Self {
        Self { state: S::Idle }
    }

    pub fn is_moving(&self) -> bool {
        match self.state {
            S::Idle
            | S::WaitingStopAnimation
            | S::StartedStopAnimation => false,
            _ => true,
        }
    }

    pub fn start_movt(&mut self) {
        self.state = S::WaitingAnimation
    }

    pub fn stop(&mut self) {
        self.state = S::WaitingStopAnimation
    }
}

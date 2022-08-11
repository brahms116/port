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
    pub is_forward: bool, // should add direction? maybe
}

impl SquishMovement {
    pub fn new() -> Self {
        Self {
            state: S::Idle,
            is_forward: true,
        }
    }

    pub fn is_moving_forward(&self) -> bool {
        if !self.is_forward {
            return false;
        }
        match self.state {
            S::Idle
            | S::WaitingStopAnimation
            | S::StartedStopAnimation => false,
            _ => true,
        }
    }

    pub fn is_moving_back(&self) -> bool {
        if self.is_forward {
            return false;
        }
        match self.state {
            S::Idle
            | S::WaitingStopAnimation
            | S::StartedStopAnimation => false,
            _ => true,
        }
    }

    pub fn forward(&mut self) {
        self.state = S::WaitingAnimation;
        self.is_forward = true;
    }

    pub fn back(&mut self) {
        self.state = S::WaitingAnimation;
        self.is_forward = false;
    }

    pub fn stop(&mut self) {
        self.state = S::WaitingStopAnimation
    }
}

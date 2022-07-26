use super::*;

pub struct Controller {
    pub progress: Progression,
    pub timer: LinearProgress,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            progress: Progression::BounceTimerStart,
            timer: LinearProgress::new(0),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Progression {
    BounceTimerStart,
    BounceTimerWait,
}

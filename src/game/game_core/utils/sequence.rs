use super::*;

pub struct Sequence {
    engine: LinearProgress,
    checkpoint: u32,
}

impl Sequence {
    pub fn new(duration: u32) -> Self {
        Sequence {
            engine: LinearProgress::new(duration),
            checkpoint: 0,
        }
    }

    pub fn advance_checkpoint(&mut self) {
        self.checkpoint += 1;
    }
    pub fn checkpoint(&self) -> u32 {
        self.checkpoint
    }
}

impl Animation for Sequence {
    fn duration(&self) -> u32 {
        self.engine.duration()
    }

    fn advance_frame(&mut self) {
        self.engine.advance_frame()
    }

    fn is_complete(&self) -> bool {
        self.engine.is_complete()
    }

    fn poll(&self) -> f64 {
        self.engine.poll()
    }

    fn reset(&mut self) {
        self.checkpoint = 0;
        self.engine.reset();
    }
}

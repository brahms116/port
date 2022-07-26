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

impl HasLinearProgress for Sequence {
    fn get_progress(&self) -> &LinearProgress {
        &self.engine
    }

    fn get_progress_mut(&mut self) -> &mut LinearProgress {
        &mut self.engine
    }
}

impl Animation for Sequence {
    fn advance_frame(&mut self) {
        self.engine.advance_frame()
    }

    fn reset(&mut self) {
        self.checkpoint = 0;
        self.engine.reset();
    }
}

pub trait Animation {
    fn duration(&self) -> u32;
    fn advance_frame(&mut self);
    fn is_complete(&self) -> bool;
    fn poll(&self) -> f64;
    fn reset(&mut self);
}

pub struct LinearAnimation {
    duration: u32,
    elasped_frames: u32,
    completed: bool,
}

impl LinearAnimation {
    pub fn new(duration: u32) -> Self {
        LinearAnimation {
            duration,
            elasped_frames: 0,
            completed: false,
        }
    }
}

impl Animation for LinearAnimation {
    fn duration(&self) -> u32 {
        self.duration
    }

    fn advance_frame(&mut self) {
        if self.elasped_frames < self.duration {
            self.elasped_frames += 1;
        } else {
            self.completed = true;
        }
    }

    fn is_complete(&self) -> bool {
        self.completed
    }

    fn poll(&self) -> f64 {
        if self.elasped_frames >= self.duration {
            return 1.0;
        }
        f64::try_from(self.elasped_frames).unwrap()
            / f64::try_from(self.duration).unwrap()
    }

    fn reset(&mut self) {
        self.elasped_frames = 0;
        self.completed = false;
    }
}

pub struct Checkpoint(u32);

impl Checkpoint {
    pub fn new() -> Self {
        Checkpoint(0)
    }
    pub fn advance_checkpoint(&mut self) {
        self.0 += 1;
    }

    pub fn checkpoint(&self) -> u32 {
        self.0
    }
}

pub struct Sequence {
    engine: LinearAnimation,
    checkpoint: u32,
}

impl Sequence {
    pub fn new(duration: u32) -> Self {
        Sequence {
            engine: LinearAnimation::new(duration),
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

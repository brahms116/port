use super::*;

pub trait HasLinearProgress {
    fn get_progress(&self) -> &LinearProgress;
    fn get_progress_mut(&mut self) -> &mut LinearProgress;
}

pub struct LinearProgress {
    duration: u32,
    elasped_frames: u32,
    completed: bool,
}

impl LinearProgress {
    pub fn new(duration: u32) -> Self {
        LinearProgress {
            duration,
            elasped_frames: 0,
            completed: false,
        }
    }
}

impl AnimationStatus for LinearProgress {
    fn duration(&self) -> u32 {
        self.duration
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
}

impl Animation for LinearProgress {
    fn advance_frame(&mut self) {
        if self.elasped_frames < self.duration {
            self.elasped_frames += 1;
        } else {
            self.completed = true;
        }
    }

    fn reset(&mut self) {
        self.elasped_frames = 0;
        self.completed = false;
    }
}

impl<T: HasLinearProgress> AnimationStatus for T {
    fn duration(&self) -> u32 {
        self.get_progress().duration()
    }

    fn is_complete(&self) -> bool {
        self.get_progress().is_complete()
    }

    fn poll(&self) -> f64 {
        self.get_progress().poll()
    }
}

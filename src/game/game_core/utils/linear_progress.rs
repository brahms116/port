use super::*;

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

impl Animation for LinearProgress {
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

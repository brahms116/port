pub trait Animation {
    fn new(duration: u32) -> Self;
    fn duration(&self) -> u32;
    fn advance_frame(&mut self);
    fn is_complete(&self) -> bool;
    fn poll(&mut self) -> f64;
    fn reset(&mut self);
}

pub struct LinearAnimation {
    duration: u32,
    elasped_frames: u32,
    completed: bool,
}

impl Animation for LinearAnimation {
    fn new(duration: u32) -> Self {
        LinearAnimation {
            duration,
            elasped_frames: 0,
            completed: false,
        }
    }

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

    fn poll(&mut self) -> f64 {
        if self.elasped_frames >= self.duration {
            return 1.0;
        }
        f64::try_from(self.elasped_frames).unwrap() / f64::try_from(self.duration).unwrap()
    }

    fn reset(&mut self) {
        self.elasped_frames = 0;
        self.completed = false;
    }
}

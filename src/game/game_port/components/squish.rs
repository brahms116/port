use super::*;

pub struct SquishConfig {
    pub start_height: f64,
    pub start_width: f64,
    pub squish_height: f64,
    pub squish_width: f64,
    pub finish_height: f64,
    pub finish_width: f64,
    pub should_anchor: bool,
}

pub enum SquishDirection {
    Front,
    Back,
    Left,
    Right,
}

pub struct SquishAnimation {
    pub config: SquishConfig,
    pub direction: SquishDirection,
    pub is_active: bool,
    pub engine: LinearProgress,
}

impl SquishAnimation {
    pub fn new(config: SquishConfig, length: u32) -> Self {
        Self {
            config,
            is_active: false,
            direction: SquishDirection::Front,
            engine: LinearProgress::new(length),
        }
    }

    fn start(&mut self) {
        self.engine.reset();
        self.is_active = true;
    }

    pub fn front(mut self) -> Self {
        self.start();
        self.direction = SquishDirection::Front;
        self
    }
    pub fn front_mut(&mut self) {
        self.start();
        self.direction = SquishDirection::Front;
    }
    pub fn left(mut self) -> Self {
        self.start();
        self.direction = SquishDirection::Left;
        self
    }
    pub fn left_mut(&mut self) {
        self.start();
        self.direction = SquishDirection::Left;
    }
    pub fn right(mut self) -> Self {
        self.start();
        self.direction = SquishDirection::Right;
        self
    }
    pub fn right_mut(&mut self) {
        self.start();
        self.direction = SquishDirection::Right;
    }
    pub fn back(mut self) -> Self {
        self.start();
        self.direction = SquishDirection::Back;
        self
    }
    pub fn back_mut(&mut self) {
        self.start();
        self.direction = SquishDirection::Back;
    }
}

impl HasLinearProgress for SquishAnimation {
    fn get_progress(&self) -> &LinearProgress {
        &self.engine
    }

    fn get_progress_mut(&mut self) -> &mut LinearProgress {
        &mut self.engine
    }
}

impl Animation for SquishAnimation {
    fn advance_frame(&mut self) {
        if self.engine.is_complete() {
            self.reset();
            self.is_active = false;
        } else {
            self.engine.advance_frame();
        }
    }

    fn reset(&mut self) {
        self.engine.reset();
    }
}

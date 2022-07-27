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

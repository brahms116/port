use super::*;

pub enum FadeAnimationType {
    In,
    Out,
}

pub struct FadeAnimation {
    pub animation_type: FadeAnimationType,
    pub engine: LinearProgress,
    pub is_active: bool,
}

impl HasLinearProgress for FadeAnimation {
    fn get_progress(&self) -> &LinearProgress {
        &self.engine
    }

    fn get_progress_mut(&mut self) -> &mut LinearProgress {
        &mut self.engine
    }
}

impl Animation for FadeAnimation {
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

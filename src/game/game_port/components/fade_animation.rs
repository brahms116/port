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

impl FadeAnimation {
    pub fn new(length: u32) -> Self {
        FadeAnimation {
            animation_type: FadeAnimationType::In,
            is_active: false,
            engine: LinearProgress::new(length),
        }
    }

    pub fn fade_in(mut self) -> Self {
        self.animation_type = FadeAnimationType::In;
        self.engine.reset();
        self.is_active = true;
        self
    }

    pub fn fade_in_mut(&mut self) {
        self.animation_type = FadeAnimationType::In;
        self.engine.reset();
        self.is_active = true;
    }
    pub fn fade_out(mut self) -> Self {
        self.animation_type = FadeAnimationType::Out;
        self.engine.reset();
        self.is_active = true;
        self
    }
    pub fn fade_out_mut(&mut self) {
        self.animation_type = FadeAnimationType::Out;
        self.engine.reset();
        self.is_active = true;
    }
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

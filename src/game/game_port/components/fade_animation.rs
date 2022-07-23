use super::*;

pub enum FadeAnimationType {
    In,
    Out,
}

pub struct FadeAnimationConfig {
    pub duration: u64,
}

pub struct FadeAnimation {
    pub animation_type: FadeAnimationType,
    pub engine: LinearProgress,
}

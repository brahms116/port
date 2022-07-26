pub trait Animation: AnimationStatus {
    fn advance_frame(&mut self);
    fn reset(&mut self);
}

pub trait AnimationStatus {
    fn duration(&self) -> u32;
    fn is_complete(&self) -> bool;
    fn poll(&self) -> f64;
}

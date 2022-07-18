pub trait Animation {
    fn duration(&self) -> u32;
    fn advance_frame(&mut self);
    fn is_complete(&self) -> bool;
    fn poll(&self) -> f64;
    fn reset(&mut self);
}

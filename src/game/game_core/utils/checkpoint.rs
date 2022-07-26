pub struct Checkpoint(u32);

impl Checkpoint {
    pub fn new() -> Self {
        Checkpoint(0)
    }
    pub fn advance_checkpoint(&mut self) {
        self.0 += 1;
    }

    pub fn checkpoint(&self) -> u32 {
        self.0
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }
}

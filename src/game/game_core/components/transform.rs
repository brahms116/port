use super::*;

pub trait Transformable {
    fn apply(self, transform: &Transform) -> Self;
}

#[derive(Default, Clone, Debug)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f64,
}

impl Transform {
    pub fn new(position: Vec2, rotation: f64) -> Self {
        Self { position, rotation }
    }
}

impl Transformable for Transform {
    fn apply(mut self, transform: &Transform) -> Self {
        let resolved = transform.position
            + self.position.rotate_deg(transform.rotation);

        self.rotation += transform.rotation;
        self.position = resolved;
        self
    }
}

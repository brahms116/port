use super::*;

#[derive(Clone, Debug)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
    pub transform: Transform,
}

impl Rect {
    pub fn two_point_flat(x1y1: Vec2, x2y2: Vec2) -> Self {
        let width = x2y2.x - x1y1.x;
        let height = x2y2.y - x1y1.y;

        Self {
            height,
            width,
            transform: Transform::new(
                Vec2::new(
                    width / 2.0 + x1y1.x,
                    height / 2.0 + x1y1.y,
                ),
                0.0,
            ),
        }
    }
    pub fn corners(&self) -> (Vec2, Vec2, Vec2, Vec2) {
        let x = self.width / 2.0;
        let y = self.height / 2.0;

        (
            Vec2::new(-x, -y).apply(&self.transform),
            Vec2::new(-x, y).apply(&self.transform),
            Vec2::new(x, y).apply(&self.transform),
            Vec2::new(x, -y).apply(&self.transform),
        )
    }
}

impl Transformable for Rect {
    fn apply(mut self, transform: &Transform) -> Self {
        self.transform = self.transform.apply(transform);
        self
    }
}

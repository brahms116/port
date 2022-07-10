use super::*;

pub struct BoxCollider {
    pub width: f64,
    pub height: f64,
    pub position: Vec2,
}

impl BoxCollider {
    pub fn new(
        width: f64,
        height: f64,
        center: Vec2,
    ) -> Self {
        Self {
            width,
            height,
            position: center,
        }
    }

    pub fn rect(&self) -> Rect {
        let x = self.width / 2.0;
        let y = self.height / 2.0;
        Rect {
            x2y2: Vec2::new(x, y),
            x1y1: Vec2::new(-x, -y),
        }
    }
}

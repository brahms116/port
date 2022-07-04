use std::ops;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn y() -> Vec2 {
        Vec2 { y: 1.0, x: 0.0 }
    }

    pub fn x() -> Vec2 {
        Vec2 { y: 0.0, x: 1.0 }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2::new(0.0, 0.0)
    }
}

pub struct Surface {
    pub points: Vec<Vec2>,
    pub color: String,
}

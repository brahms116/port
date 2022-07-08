use std::ops;

#[derive(Clone, Copy, Debug)]
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

    pub fn angle(&self) -> f64 {
        (self.y / self.x).atan()
    }

    pub fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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

impl ops::Add<&Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<Vec2> for &Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn add(self, rhs: &Vec2) -> Vec2 {
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

impl ops::Sub<&Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<Vec2> for &Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: &Vec2) -> Self::Output {
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

impl ops::Mul<f64> for &Vec2 {
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

#[derive(Clone)]
pub struct Rect {
    pub x1y1: Vec2,
    pub x2y2: Vec2,
}

impl Rect {
    pub fn corners(&self) -> (Vec2, Vec2, Vec2, Vec2) {
        (
            self.x1y1,
            Vec2::new(self.x1y1.x, self.x2y2.y),
            self.x2y2,
            Vec2::new(self.x2y2.x, self.x1y1.y),
        )
    }
    pub fn check_point_in_rect(&self, point: Vec2) -> bool {
        let x1 = self.x1y1.x;
        let y1 = self.x1y1.y;
        let x2 = self.x2y2.x;
        let y2 = self.x2y2.y;

        let x = point.x;
        let y = point.y;

        let in_x = x2 > x && x1 < x || x1 > x && x2 < x;
        let in_y = y2 > y && y1 < y || y1 > y && y2 < y;

        in_x && in_y
    }
    pub fn check_collision(static_rect: &Rect, moving_rect: &Rect, vel_vec: Vec2) -> Option<Vec2> {
        let (v1, v2, v3, v4) = moving_rect.corners();
        for i in [v1, v2, v3, v4] {
            if static_rect.check_point_in_rect(i) {
                //TODO lol hella fix this please
                return Some(-vel_vec);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Surface {
    pub points: Vec<Vec2>,
    pub color: RGBA,
}

#[derive(Debug, Clone)]
pub struct RGBA {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: f64,
}

impl std::fmt::Display for RGBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

impl RGBA {
    pub fn new(r: u32, g: u32, b: u32, a: f64) -> RGBA {
        RGBA { r, g, b, a }
    }
}

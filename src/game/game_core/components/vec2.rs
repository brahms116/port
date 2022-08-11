use super::*;
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub enum Quadrant {
    Zero,
    X,
    Quad1,
    Y,
    Quad2,
    NegX,
    Quad3,
    NegY,
    Quad4,
}

const TO_DEGREES: f64 = 180.0 / std::f64::consts::PI;
const TO_RADIANS: f64 = std::f64::consts::PI / 180.0;

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

    pub fn unit(&self) -> Vec2 {
        if self.mag() == 0.0 {
            return Self::default();
        } else {
            self * (1.0 / self.mag())
        }
    }

    pub fn angle(&self, vec: Self) -> f64 {
        let value = self.dot(vec) / self.mag() / vec.mag();
        value.acos()
    }

    pub fn angle_deg(&self, vec: Self) -> f64 {
        self.angle(vec).to_degrees()
    }

    pub fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn dot(&self, vec: Self) -> f64 {
        self.x * vec.x + self.y * vec.y
    }

    pub fn rotate(mut self, radians: f64) -> Self {
        let x = self.x * radians.cos()
            + self.y * -radians.sin();
        let y =
            self.x * radians.sin() + self.y * radians.cos();
        self.x = x;
        self.y = y;
        self
    }

    pub fn rotate_deg(self, degrees: f64) -> Self {
        let radians = degrees.to_radians();
        self.rotate(radians)
    }

    pub fn perpendicular(&self) -> Self {
        if self.mag() == 0.0 {
            return Self::default();
        }

        if self.x != 0.0 {
            Self::new(-self.y / self.x, 1.0).unit()
        } else {
            Self::new(1.0, -self.x / self.y).unit()
        }
    }

    pub fn quadrant(&self) -> Quadrant {
        if self.x == 0.0 && self.y == 0.0 {
            return Quadrant::Zero;
        }

        if self.x == 0.0 && self.y > 0.0 {
            return Quadrant::Y;
        }

        if self.x == 0.0 && self.y < 0.0 {
            return Quadrant::NegY;
        }

        if self.x > 0.0 && self.y == 0.0 {
            return Quadrant::X;
        }

        if self.x < 0.0 && self.y == 0.0 {
            return Quadrant::NegX;
        }

        if self.x > 0.0 && self.y > 0.0 {
            return Quadrant::Quad1;
        }

        if self.x < 0.0 && self.y > 0.0 {
            return Quadrant::Quad2;
        }

        if self.x < 0.0 && self.y < 0.0 {
            return Quadrant::Quad3;
        }

        if self.x > 0.0 && self.y < 0.0 {
            return Quadrant::Quad4;
        }

        Quadrant::Zero
    }

    pub fn rotation(&self) -> f64 {
        let quad = self.quadrant();

        if let Quadrant::Zero = quad {
            return 0.0;
        }

        if let Quadrant::X = quad {
            return -90.0;
        }

        if let Quadrant::NegX = quad {
            return 90.0;
        }

        if let Quadrant::Y = quad {
            return 0.0;
        }

        if let Quadrant::NegY = quad {
            return 0.0;
        }

        if let Quadrant::Quad1 = quad {
            return -((self.x / self.y).atan()
                * TO_DEGREES);
        }

        if let Quadrant::Quad2 = quad {
            return (-self.x / self.y).atan() * TO_DEGREES;
        }

        if let Quadrant::Quad3 = quad {
            return 180.0
                - (-self.x / -self.y).atan() * TO_DEGREES;
        }

        if let Quadrant::Quad4 = quad {
            return -(180.0
                - (self.x / -self.y).atan() * TO_DEGREES);
        }
        0.0
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

impl Transformable for Vec2 {
    fn apply(mut self, transform: &Transform) -> Self {
        self = self.rotate_deg(transform.rotation);
        self += transform.position;
        self
    }
}

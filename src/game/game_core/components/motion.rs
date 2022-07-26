use super::*;
#[derive(Default, Debug)]
pub struct Motion {
    pub vel: Vec2,
    pub accel: Vec2,
    pub angular_vel: f64,
    pub angular_accel: f64,
}

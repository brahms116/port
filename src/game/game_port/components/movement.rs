use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
pub enum MovementDirection {
    Front,
    Idle,
    Back,
    Left,
    Right,
}

pub struct TravelSettings(
    pub HashMap<MovementDirection, TravelConfig>,
);

pub struct TravelConfig {
    pub max_vel: f64,
    pub travel_accel: f64,
}

pub struct Movement {
    pub direction: MovementDirection,
    pub settings: TravelSettings,
    pub applied_accel: f64,
}

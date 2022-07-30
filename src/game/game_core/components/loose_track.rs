use super::*;

pub struct LooseTracking {
    pub target: Option<Entity>,
    pub radius: f64,
    pub inner_travel_vel: f64,
    pub inner_rotation_vel: f64,
    pub outer_rotation_vel: f64,
}

impl LooseTracking {}

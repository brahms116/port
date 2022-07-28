use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
pub enum MovementDirection {
    Front,
    Idle,
    Back,
    Left,
    Right,
}

pub struct TravelSettings {
    pub front: TravelConfig,
    pub back: TravelConfig,
    pub left: TravelConfig,
    pub right: TravelConfig,
}

impl TravelSettings {
    pub fn uniform_config(config: &TravelConfig) -> Self {
        Self {
            front: config.clone(),
            back: config.clone(),
            left: config.clone(),
            right: config.clone(),
        }
    }
}

#[derive(Clone)]
pub struct TravelConfig {
    pub max_vel: f64,
    pub travel_accel: f64,
}

pub struct Movement {
    pub direction: MovementDirection,
    pub settings: TravelSettings,
    pub applied_accel: f64,
}

impl Movement {
    pub fn new(config: TravelSettings) -> Self {
        Self {
            direction: MovementDirection::Idle,
            settings: config,
            applied_accel: 0.0,
        }
    }

    pub fn forward(mut self) -> Self {
        self.direction = MovementDirection::Front;
        self.applied_accel = 0.0;
        self
    }
    pub fn forward_mut(&mut self) {
        self.direction = MovementDirection::Front;
        self.applied_accel = 0.0;
    }
    pub fn back(mut self) -> Self {
        self.direction = MovementDirection::Back;
        self.applied_accel = 0.0;
        self
    }
    pub fn back_mut(&mut self) {
        self.direction = MovementDirection::Back;
        self.applied_accel = 0.0;
    }
    pub fn left(mut self) -> Self {
        self.direction = MovementDirection::Left;
        self.applied_accel = 0.0;
        self
    }
    pub fn left_mut(&mut self) {
        self.direction = MovementDirection::Left;
        self.applied_accel = 0.0;
    }
    pub fn right(mut self) -> Self {
        self.direction = MovementDirection::Right;
        self.applied_accel = 0.0;
        self
    }
    pub fn right_mut(&mut self) {
        self.direction = MovementDirection::Right;
        self.applied_accel = 0.0;
    }
    pub fn stop_mut(&mut self) {
        self.direction = MovementDirection::Idle;
        self.applied_accel = 0.0;
    }
}

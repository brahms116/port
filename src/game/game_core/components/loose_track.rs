use super::*;

/// A component which enables an entity to loosely track another entity's position
pub struct LooseTracking {
    /// The target entity which to track
    pub target: Option<Entity>,
    /// The maximum distance which the target is allowed to be from the source
    pub radius: f64,
    /// When the source is moved by the target, the ratio of the target's velocity the source will
    /// travel in the direction perpendicular to the target's motion
    pub perpendicular_ratio: f64,
    /// Should the tracking be active
    pub is_active: bool,
}

impl LooseTracking {}

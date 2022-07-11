mod box_collider;
mod cbs;
mod collision_markers;
mod globals;
mod motion;
mod player_state;
mod transform;

pub use box_collider::*;
pub use cbs::*;
pub use collision_markers::*;
pub use globals::*;
pub use motion::*;
pub use player_state::*;
pub use transform::*;

use super::*;

impl Transformable for Vec2 {
    fn apply(
        mut self,
        transform: &Transform,
    ) -> Self {
        self =
            self.rotate_deg(transform.rotation);
        self += transform.position;
        self
    }
}

impl Transformable for Rect {
    fn apply(
        mut self,
        transform: &Transform,
    ) -> Self {
        let x1y1 = self
            .x1y1
            .rotate_deg(transform.rotation);
        let x2y2 = self
            .x2y2
            .rotate_deg(transform.rotation);
        self.x1y1 = x1y1 + transform.position;
        self.x2y2 = x2y2 + transform.position;
        self
    }
}

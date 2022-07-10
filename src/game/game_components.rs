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

impl Transformable for Rect {
    // TODO Apply rotation here
    fn apply(
        mut self,
        transform: &Transform,
    ) -> Self {
        self.x1y1 += transform.position;
        self.x2y2 += transform.position;
        self
    }
}

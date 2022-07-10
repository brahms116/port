mod box_collider;
mod globals;
mod motion;
mod player_state;
mod state_cbs;
mod transform;

pub use box_collider::*;
pub use globals::*;
pub use motion::*;
pub use player_state::*;
pub use state_cbs::*;
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

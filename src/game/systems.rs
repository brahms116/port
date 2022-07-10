mod collision;
mod entity_state;
mod game_control;
mod motion;
mod render;

use super::*;
use hecs::*;

pub use collision::*;
pub use entity_state::*;
pub use game_control::*;
pub use motion::*;
pub use render::*;

pub fn collision_player(
    state: &mut PlayerState,
    transform: &mut Transform,
    motion: &mut Motion,
    correction_vec: Vec2,
) {
    *state = PlayerState::post_motion(
        PlayerDirection::Front,
    );
    motion.vel = Vec2::default();
    motion.accel = Vec2::default();
    transform.position += correction_vec;
}

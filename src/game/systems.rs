use super::game_api::*;
use super::game_components::*;
use super::game_core::*;
use hecs::*;

pub fn system_update_frame(n: &mut FrameCount) {
    n.0 += 1;
}

pub fn get_camera_pos(world: &World) -> Vec2 {
    for (_id, (_cam, pos)) in &mut world.query::<(&Camera, &Position)>() {
        return pos.0;
    }
    return Vec2::default();
}

pub fn system_render<T: GameApi>(
    pos: &Position,
    rot: &Rotation,
    surfaces: &Surfaces,
    api: &T,
    camera_pos: Vec2,
) {
    for surface in &surfaces.0 {
        let points: Vec<Vec2> = surface.points.iter().map(|e| *e + pos.0).collect();
        /* Need to apply rotation */
    }
}

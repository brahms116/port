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

pub fn system_render<T: GameApi, E>(
    pos: &Position,
    _rot: Option<&Rotation>,
    surfaces: &Surfaces<E>,
    evs: &mut E,
    api: &T,
    camera_pos: Vec2,
) {
    for surface in &surfaces.0(evs, api.window_size()) {
        let points: Vec<Vec2> = surface.points.iter().map(|e| *e + pos.0).collect();
        /* Need to apply rotation */

        let points: Vec<Vec2> = points
            .iter()
            .map(|e| map_vec2(e, &camera_pos, api.window_size()))
            .collect();

        let screen_surface = Surface {
            points,
            color: surface.color.clone(),
        };

        api.draw_surface(screen_surface)
    }
}

pub fn system_movement(pos: &mut Position, _rot: Option<&Rotation>, movt: &MovementDelta) {
    /* TODO  Need to apply rotation here */
    pos.0 = pos.0 + movt.0
}

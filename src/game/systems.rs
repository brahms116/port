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
    _rot: Option<&Rotation>,
    width: Option<&Width>,
    height: Option<&Height>,
    scale_x: Option<&ScaleX>,
    scale_y: Option<&ScaleY>,
    opacity: Option<&Opacity>,
    surfaces: &Surfaces,
    api: &T,
    camera_pos: Vec2,
) {
    for surface in &surfaces.0(SurfacesArgs {
        width: width.map(|e| e.0),
        height: height.map(|e| e.0),
        scale_x: scale_x.map(|e| e.0),
        scale_y: scale_y.map(|e| e.0),
        opacity: opacity.map(|e| e.0),
    }) {
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

pub fn system_opacity(opt: &mut Opacity, delta: &OpacityDelta) {
    opt.0 += delta.0;
    if opt.0 < 0.0 {
        opt.0 = 0.0
    };
    if opt.0 > 1.0 {
        opt.0 = 1.0
    };
}

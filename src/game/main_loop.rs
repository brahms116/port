use super::game_api::*;
use super::game_components::*;
use super::systems::*;
use hecs::*;

pub fn main_loop<T: GameApi>(world: &mut World, api: &T) {
    /* Movement */
    for (_id, (pos, rot, movt)) in
        world.query_mut::<(&mut Position, Option<&Rotation>, &MovementDelta)>()
    {
        system_movement(pos, rot, movt);
    }

    /* Opacity */
    for (_id, (opt, delta)) in world.query_mut::<(&mut Opacity, &OpacityDelta)>() {
        system_opacity(opt, delta)
    }

    /* Rendering */
    let camera_pos = get_camera_pos(world);
    for (_id, (pos, rot, w, h, s_x, s_y, opt, surfaces)) in world.query_mut::<(
        &Position,
        Option<&Rotation>,
        Option<&Width>,
        Option<&Height>,
        Option<&ScaleX>,
        Option<&ScaleY>,
        Option<&Opacity>,
        &Surfaces,
    )>() {
        system_render(pos, rot, w, h, s_x, s_y, opt, surfaces, api, camera_pos);
    }

    /* Game Controller */
    for (_id, (frame_count, _stage_count)) in world.query_mut::<(&mut FrameCount, &StageCount)>() {
        system_update_frame(frame_count);
    }
}

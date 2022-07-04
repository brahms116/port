use super::game_api::*;
use super::game_components::*;
use super::systems::*;
use hecs::*;

pub fn main_loop<T: GameApi>(world: &mut World, api: &T) {
    /* Rendering */
    let camera_pos = get_camera_pos(world);
    for (_id, (pos, rot, surfaces)) in world.query_mut::<(&Position, &Rotation, &Surfaces)>() {
        system_render(pos, rot, surfaces, api, camera_pos);
    }

    /* Game Controller */
    for (_id, (frame_count, _stage_count)) in world.query_mut::<(&mut FrameCount, &StageCount)>() {
        api.log(&format!("{}", frame_count.0));
        system_update_frame(frame_count);
    }
}

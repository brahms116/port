use super::*;
use hecs::*;

pub fn main_loop<T: GameApi>(world: &mut World, api: &T) {
    /* Globals */
    let camera_pos = get_camera_pos(world);

    /* Movement */
    for (_id, (pos, rot, movt)) in
        world.query_mut::<(&mut Position, Option<&Rotation>, &MovementDelta)>()
    {
        system_movement(pos, rot, movt);
    }

    /* Update animations */
    for (_id, (evs, _)) in world.query_mut::<(&mut PSEVS, Option<()>)>() {
        evs.update();
    }

    /* Rendering */

    for (_id, (pos, rot, surfaces)) in
        world.query_mut::<(&Position, Option<&Rotation>, &Surfaces<()>)>()
    {
        system_render(pos, rot, surfaces, &mut (), api, camera_pos);
    }

    for (_id, (pos, rot, surfaces, evs)) in
        world.query_mut::<(&Position, Option<&Rotation>, &Surfaces<PSEVS>, &mut PSEVS)>()
    {
        system_render(pos, rot, surfaces, evs, api, camera_pos);
    }

    /* Game Controller */
    for (_id, (frame_count, _stage_count)) in world.query_mut::<(&mut FrameCount, &StageCount)>() {
        system_update_frame(frame_count);
    }
}

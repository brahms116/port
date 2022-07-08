use super::*;
use hecs::*;

pub fn main_loop<T: GameApi>(world: &mut World, api: &T) {
    /* Globals */
    let camera_transform = get_camera_transform(world);

    /* Entity States */
    for (_id, (player, random)) in world.query_mut::<(Option<&mut PlayerState>, Option<()>)>() {
        if let Some(n) = player {
            n.update()
        }
    }

    /* Entity State Motion System */
    for (_id, (transform, motion, player)) in world.query_mut::<(
        &mut Transform,
        &mut Motion,
        (
            Option<&mut PlayerState>,
            Option<&StateMotionCb<PlayerState>>,
        ),
    )>() {
        if let Some(state) = player.0 {
            if let Some(cb) = player.1 {
                cb.0(state, motion, transform)
            }
        }
    }

    /* Movement */
    for (_id, (motion, transform)) in world.query_mut::<(&mut Motion, Option<&mut Transform>)>() {
        system_motion(motion, transform)
    }

    /* Render System */
    for (_id, (transform, player)) in world.query_mut::<(
        &Transform,
        (
            Option<&mut PlayerState>,
            Option<&StateRenderCb<PlayerState>>,
        ),
    )>() {
        if let Some(state) = player.0 {
            if let Some(render) = player.1 {
                system_render(
                    transform,
                    render.0(state, api.window_size()),
                    api,
                    &camera_transform,
                );
            }
        }
    }

    /* Game Controller */
    for (_id, (frame_count, _stage_count)) in world.query_mut::<(&mut FrameCount, &StageCount)>() {
        system_update_frame(frame_count);
    }
}

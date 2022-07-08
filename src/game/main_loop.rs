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
        Option<(&mut PlayerState, &StateMotionCb<PlayerState>)>,
    )>() {
        if let Some(p) = player {
            p.1 .0(p.0, motion, transform)
        }
    }

    /* Movement */
    for (_id, (motion, transform)) in world.query_mut::<(&mut Motion, Option<&mut Transform>)>() {
        system_motion(motion, transform)
    }

    /* Collision */
    for (id, (transform, collider)) in &mut world.query::<(&Transform, &BoxCollider)>() {
        let collider_box = collider.rect().apply(transform);
        let player_id: Option<Entity> = None;
        /* Player Collision */
        for (_id, (p_transform, player, p_collider, p_motion)) in &mut world.query::<(
            &Transform,
            &PlayerState,
            &StateColliderCb<PlayerState>,
            &Motion,
        )>() {
            let p_box = p_collider.0(player).rect().apply(p_transform);
        }
    }

    /* Render System */
    for (_id, (transform, surface)) in world.query_mut::<(&Transform, &RenderStatic)>() {
        system_render(transform, &surface.0, api, &camera_transform)
    }

    for (_id, (transform, player)) in world.query_mut::<(
        &Transform,
        Option<(&mut PlayerState, &StateRenderCb<PlayerState>)>,
    )>() {
        if let Some(p) = player {
            system_render(
                transform,
                &p.1 .0(p.0, api.window_size()),
                api,
                &camera_transform,
            )
        }
    }

    /* Game Controller */
    for (_id, (frame_count, _stage_count)) in world.query_mut::<(&mut FrameCount, &StageCount)>() {
        system_update_frame(frame_count);
    }
}

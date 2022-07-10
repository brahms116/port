use super::*;
use hecs::*;

/* Helpers */

pub fn update_entity_state<
    T: 'static + Sync + Send,
>(
    world: &mut World,
) {
    for (_id, (state, cb)) in world
        .query_mut::<(&mut T, &UpdateStateCb<T>)>(
        )
    {
        cb.0(state)
    }
}

pub fn update_entity_motion<
    T: 'static + Sync + Send,
>(
    world: &mut World,
) {
    for (_id, (state, motion, transform, cb)) in
        world.query_mut::<(
            &mut T,
            &mut Motion,
            &mut Transform,
            &StateMotionCb<T>,
        )>()
    {
        cb.0(state, motion, transform);
    }
}

fn render_entity_with_state<
    T: 'static + Send + Sync,
    K: GameApi,
>(
    api: &K,
    world: &mut World,
    camera_transform: &Transform,
) {
    for(_id,(state,transform,cb,)) in 
        world.query_mut::<(
            &T,
            &Transform,
            &StateRenderCb<T>
        )>(){
            system_render(transform,&cb.0(state,api.window_size()),api,camera_transform)
        }
}

/* Main loop */
pub fn main_loop<T: GameApi>(
    world: &mut World,
    api: &T,
) {
    /* Globals */
    let camera_transform =
        get_camera_transform(world);

    /* Entity States */
    update_entity_state::<PlayerState>(world);

    /* Entity State Motion System */
    update_entity_motion::<PlayerState>(world);

    /* Movement */
    for (_id, (motion, transform)) in world.query_mut::<(&mut Motion, Option<&mut Transform>)>() {
        system_motion(motion, transform)
    }

    /* Collision */
    for (_id, (transform, collider)) in &mut world
        .query::<(&Transform, &BoxCollider)>()
    {
        let collider_box =
            collider.rect().apply(transform);
        let mut player_id: Option<Entity> = None;
        let mut correction: Option<Vec2> = None;
        /* Player Collision */
        for (
            p_id,
            (
                p_transform,
                player,
                p_collider,
                p_motion,
            ),
        ) in &mut world.query::<(
            &Transform,
            &PlayerState,
            &StateColliderCb<PlayerState>,
            &Motion,
        )>() {
            let p_box = p_collider.0(player)
                .rect()
                .apply(p_transform);
            let res = Rect::check_collision(
                &collider_box,
                &p_box,
                p_motion.vel,
            );
            if let Some(v) = res {
                player_id = Some(p_id);
                correction = Some(v);
            }
            break;
        }
        if let Some(p_id) = player_id {
            let correction = correction.unwrap();
            let mut p_transform = world
                .get_mut::<Transform>(p_id)
                .unwrap();
            let mut p_motion = world
                .get_mut::<Motion>(p_id)
                .unwrap();
            let mut p_state = world
                .get_mut::<PlayerState>(p_id)
                .unwrap();
            collision_player(
                &mut *p_state,
                &mut *p_transform,
                &mut *p_motion,
                correction,
            )
        }
    }

    /* Render System */
    for (_id, (transform, surface)) in world
        .query_mut::<(&Transform, &RenderStatic)>(
        )
    {
        system_render(
            transform,
            &surface.0,
            api,
            &camera_transform,
        )
    }

    render_entity_with_state::<PlayerState, T>(
        api,
        world,
        &camera_transform,
    );

    /* Game Controller */
    for (_id, (frame_count, _stage_count)) in world.query_mut::<(&mut FrameCount, &StageCount)>() {
        system_update_frame(frame_count);
    }
}

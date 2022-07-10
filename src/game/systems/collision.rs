use super::*;

pub fn collision_system(world: &mut World) {
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
}

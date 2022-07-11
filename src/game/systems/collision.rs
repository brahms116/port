use super::*;

pub fn collision_system<
    Static: StaticCollisionMarker + Send + Sync + 'static,
    DynamicState: Send + Sync + 'static,
>(
    world: &mut World,
) {
    for (_id, (transform, collider, _marker)) in
        &mut world.query::<(
            &Transform,
            &BoxCollider,
            &Static,
        )>()
    {
        /* We get the static box's collider rect */
        let static_box =
            collider.rect().apply(transform);

        // TODO store these as an Vec
        let mut entity_id: Option<Entity> = None;
        let mut correction_vec: Option<Vec2> =
            None;

        /* Then we check for all the instances of the given moving entity type and check for
         * collisions */
        for (
            d_id,
            (
                d_transform,
                d,
                d_collider_cb,
                d_motion,
            ),
        ) in &mut world.query::<(
            &Transform,
            &DynamicState,
            &StateColliderCb<DynamicState>,
            &Motion,
        )>() {
            let d_box = d_collider_cb.0(d)
                .rect()
                .apply(d_transform);
            web_sys::console::log_1(
                &format!("Player transform picked up {:?}", d_box).into(),
            );
            let res = Rect::check_collision(
                &static_box,
                &d_box,
                d_motion.vel.rotate_deg(
                    d_transform.rotation,
                ),
            );
            if let Some(v) = res {
                entity_id = Some(d_id);
                correction_vec = Some(v);
            }
            //TODO store in vec and continue searching for all ids
            break;
        }

        /* If there is a collision with this static box */
        if entity_id.is_some()
            && correction_vec.is_some()
        {
            let entity_id = entity_id.unwrap();
            let correction_vec =
                correction_vec.unwrap();
            let cb =
                world.get::<CollisionCb<
                    Static,
                    DynamicState,
                >>(entity_id);
            if let Ok(cb) = cb {
                web_sys::console::log_1(
                    &format!(
                        "correction vec passed in {:?}",
                        correction_vec
                    )
                    .into(),
                );
                cb.0(
                    entity_id,
                    world,
                    &correction_vec,
                );
            }
        }
    }
}

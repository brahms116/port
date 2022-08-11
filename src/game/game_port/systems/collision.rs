use super::*;
use std::collections::HashMap;

pub fn get_correction_vec(
    static_box: &Vec<Vec2>,
    dynamic_box: &Vec<Vec2>,
    travel_vec: Vec2,
) -> Vec2 {
    if travel_vec.mag() == 0.0 {
        return Vec2::default();
    }

    let correction_vec = travel_vec.unit() * -1.0;

    let mut low = 0.0;
    let mut high = travel_vec.mag();

    for _ in 0..5 {
        let try_mag = low + (high - low) / 2.0;
        let try_vec = correction_vec * try_mag;
        let new_collision_box: Vec<Vec2> = dynamic_box
            .iter()
            .map(|e| e + try_vec)
            .collect();
        if polygon_collision(static_box, &new_collision_box)
        {
            low = try_mag;
        } else {
            high = try_mag;
        }
    }

    correction_vec * high
}

pub fn get_correction_rotation(
    static_box: &Vec<Vec2>,
    dynamic_box: &Vec<Vec2>,
    rotation: f64,
    dynamic_transform: &Transform,
) -> f64 {
    if rotation == 0.0 {
        return rotation;
    }

    let cor_rotation = -rotation;

    let is_pos =
        if cor_rotation < 0.0 { false } else { true };

    let mut low = 0.0;
    let mut high = cor_rotation;

    for _ in 0..0 {
        let try_rotation = if is_pos {
            low + (high - low) / 2.0
        } else {
            low - (low - high) / 2.0
        };

        let mut try_transform = dynamic_transform.clone();
        try_transform.rotation += try_rotation;

        let new_collision_box: Vec<Vec2> = dynamic_box
            .iter()
            .map(|e| e.apply(&try_transform))
            .collect();

        if polygon_collision(static_box, &new_collision_box)
        {
            low = try_rotation;
        } else {
            high = try_rotation;
        }
    }
    high
}

pub fn system_static_collision<T: GameApi>(
    world: &mut World,
    api: &T,
) {
    let mut cor_vecs = HashMap::<Entity, Vec2>::new();
    let mut cor_rotations = HashMap::<Entity, f64>::new();

    for (static_id, (transform, collision_box, _marker)) in
        &mut world.query::<(
            &Transform,
            &CollisionBox,
            &StaticCollider,
        )>()
    {
        let static_transform = resolve_parent_transform(
            static_id, transform, world, api,
        );

        let static_collision_box = CollisionBox::new(
            collision_box
                .points
                .iter()
                .map(|e| e.apply(&static_transform))
                .collect(),
        );

        for (
            dyn_id,
            (transform, collision_box, motion, _marker),
        ) in &mut world.query::<(
            &Transform,
            &CollisionBox,
            &Motion,
            &DynamicCollider,
        )>() {
            let dynamic_transform =
                resolve_parent_transform(
                    dyn_id, transform, world, api,
                );

            let dynamic_collision_box = CollisionBox::new(
                collision_box
                    .points
                    .iter()
                    .map(|e| e.apply(&dynamic_transform))
                    .collect(),
            );

            if polygon_collision(
                &static_collision_box.points,
                &dynamic_collision_box.points,
            ) {
                api.log(&format!("collision happened",));

                api.log(&format!(
                    "static {:?}",
                    static_collision_box.points
                ));
                api.log(&format!(
                    "dyn {:?}",
                    dynamic_collision_box.points
                ));

                /* Collision happens */
                let dynamic_vel = motion
                    .vel
                    .rotate_deg(dynamic_transform.rotation);

                api.log(&format!(
                    "dyn_vel {:?}",
                    dynamic_vel
                ));

                if dynamic_vel.mag() != 0.0 {
                    let cor_vec = get_correction_vec(
                        &static_collision_box.points,
                        &dynamic_collision_box.points,
                        dynamic_vel,
                    );

                    api.log(&format!(
                        "correction vec {:?}",
                        cor_vec
                    ));

                    if let Some(v) =
                        cor_vecs.get_mut(&dyn_id)
                    {
                        if cor_vec.mag() > v.mag() {
                            *v = cor_vec
                        }
                    } else {
                        cor_vecs.insert(dyn_id, cor_vec);
                    }

                    cor_vecs.insert(dyn_id, cor_vec);
                } else {
                    let rotation_vel = motion.angular_vel;

                    api.log(&format!(
                        "rotation vel {:?}",
                        rotation_vel
                    ));

                    let cor_rotation =
                        get_correction_rotation(
                            &static_collision_box.points,
                            &dynamic_collision_box.points,
                            rotation_vel,
                            &dynamic_transform,
                        );

                    api.log(&format!(
                        "correction rotation {:?}",
                        cor_rotation
                    ));

                    if let Some(r) =
                        cor_rotations.get_mut(&dyn_id)
                    {
                        if cor_rotation > *r {
                            *r = cor_rotation;
                        }
                    } else {
                        cor_rotations
                            .insert(dyn_id, cor_rotation);
                    }
                }
            }
        }
    }

    for (id, vec) in cor_vecs.into_iter() {
        if let Ok(mut transform) =
            world.get_mut::<Transform>(id)
        {
            transform.position += vec;
        }
    }

    for (id, r) in cor_rotations.into_iter() {
        if let Ok(mut transform) =
            world.get_mut::<Transform>(id)
        {
            transform.rotation += r;
        }
    }
}

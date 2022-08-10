use super::*;

pub fn system_loose_tracking<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (id, (track,)) in
        &mut world.query::<(&LooseTracking,)>()
    {
        if track.target.is_none() || !track.is_active {
            continue;
        }

        let target = track.target.unwrap();

        let target_transform =
            world.get::<Transform>(target);
        if target_transform.is_err() {
            continue;
        }
        let target_transform = target_transform.unwrap();

        let target_motion = world.get::<Motion>(target);
        if target_motion.is_err() {
            continue;
        }
        let target_motion = target_motion.unwrap();

        let source_transform =
            world.get_mut::<Transform>(id);
        if source_transform.is_err() {
            continue;
        }
        let mut source_transform =
            source_transform.unwrap();

        let diff_vec = target_transform.position
            - source_transform.position;

        let is_inside = diff_vec.mag() < track.radius;
        source_transform.rotation = diff_vec.rotation();

        let mut perpendicular = track.perpendicular_ratio
            * target_motion.vel.mag();

        let prediction = target_transform.position
            + target_motion.vel.rotate_deg(
                target_motion.angular_vel
                    + target_motion.angular_accel
                    + target_transform.rotation,
            );

        let mut z = (prediction
            - target_transform.position)
            .perpendicular();

        let mut distance = diff_vec.dot(z);

        if distance < 0.0 {
            z = z * -1.0;
            distance = diff_vec.dot(z);
        }

        if perpendicular > distance {
            perpendicular = distance;
        }

        let opposite = distance - perpendicular;

        let side = (track.radius.powi(2)
            - opposite.powi(2))
        .powf(0.5);

        let side = target_transform.position
            + (prediction - target_transform.position)
                .unit()
                * -side;

        if !is_inside {
            source_transform.position =
                side + z * (-opposite);
        }
    }
}

use super::*;

pub fn system_loose_tracking<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (id, (track,)) in
        &mut world.query::<(&LooseTracking,)>()
    {
        if track.target.is_none() {
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

        // let source_motion = world.get_mut::<Motion>(id);
        // if source_motion.is_err() {
        //     continue;
        // }
        // let mut source_motion = source_motion.unwrap();

        let diff_vec = target_transform.position
            - source_transform.position;

        let diff_rotation = target_transform.rotation
            - source_transform.rotation;

        let is_inside = diff_vec.mag() < track.radius;

        let travel_speed = if is_inside {
            track.inner_travel_vel
        } else {
            target_motion.vel.mag()
        };

        let travel_speed = if travel_speed > diff_vec.mag()
        {
            diff_vec.mag()
        } else {
            travel_speed
        };

        let rotation_speed = if is_inside {
            track.inner_rotation_vel
        } else {
            track.outer_rotation_vel
        };

        let rotation_speed =
            if rotation_speed > diff_rotation.abs() {
                diff_rotation.abs()
            } else {
                rotation_speed
            };

        let velocity = diff_vec.unit() * travel_speed;

        let rotation = if diff_rotation >= 0.0 {
            rotation_speed
        } else {
            -rotation_speed
        };

        source_transform.position += velocity;

        // if target_motion.vel.mag() > 0.0
        //     || target_motion.angular_vel != 0.0
        // {
        source_transform.rotation += rotation;
        //}

        //source_motion.vel = velocity;
        //source_motion.angular_vel = rotation;
    }
}

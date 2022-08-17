use super::*;

pub fn system_motion<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (_id, (motion, transform)) in world
        .query_mut::<(&mut Motion, Option<&mut Transform>)>(
        )
    {
        update_motion(motion, transform)
    }
}

pub fn update_motion(
    motion: &mut Motion,
    transform: Option<&mut Transform>,
) {
    motion.vel = motion.vel + motion.accel;
    motion.angular_vel += motion.angular_accel;

    if let Some(t) = transform {
        t.rotation += motion.angular_vel;
        let true_vec = motion.vel.rotate_deg(t.rotation);
        t.position += true_vec;
    }
}

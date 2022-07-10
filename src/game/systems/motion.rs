use super::*;

pub fn entity_motion_system<
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

pub fn motion_system(world: &mut World) {
    for (_id, (motion, transform)) in world.query_mut::<(&mut Motion, Option<&mut Transform>)>() {
        update_motion(motion, transform)
    }
}

pub fn update_motion(
    motion: &mut Motion,
    transform: Option<&mut Transform>,
) {
    motion.vel = motion.vel + motion.accel;
    motion.angular_vel += motion.angular_accel;
    //TODO: apply rotation
    //TODO: Apply new or old rotation?
    if let Some(t) = transform {
        t.position = t.position + motion.vel;
        t.rotation += motion.angular_vel;
    }
}

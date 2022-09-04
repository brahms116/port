use super::*;

pub fn system_rotation<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (_id, (rot, motion)) in
        world.query_mut::<(&mut Rotation, &mut Motion)>()
    {
        let speed = rot.config.rotation_speed;
        motion.angular_accel = speed * rot.factor;
    }
}

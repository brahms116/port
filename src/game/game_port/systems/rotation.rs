use super::*;

type S = RotationState;

pub fn system_rotation<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (_id, (rot, motion)) in
        world.query_mut::<(&mut Rotation, &mut Motion)>()
    {
        let speed = rot.config.rotation_speed;
        match rot.state {
            S::Left => {
                motion.angular_vel = speed;
            }
            S::Right => {
                motion.angular_vel = -speed;
            }
            S::Idle => {
                motion.angular_vel = 0.0;
            }
        }
    }
}

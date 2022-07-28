use super::*;

type S = RotationState;

pub fn system_rotation<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (_id, (rot, motion)) in
        world.query_mut::<(&Rotation, &mut Motion)>()
    {
        match rot.state {
            S::Left => {
                motion.angular_vel = 2.0;
            }
            S::Right => {
                motion.angular_vel = -2.0;
            }
            S::Idle => {
                motion.angular_vel = 0.0;
            }
        }
    }
}

use super::*;

pub fn system_movement<T: GameApi>(
    world: &mut World,
    _api: &T,
) {
    for (_id, (movt, motion)) in
        world.query_mut::<(&mut Movement, &mut Motion)>()
    {
        let d = &movt.direction;
        let travel_accel = movt
            .settings
            .0
            .get(&d)
            .map(|e| e.travel_accel)
            .unwrap_or(0.0);
        let desired_speed = movt
            .settings
            .0
            .get(&d)
            .map(|e| e.max_vel)
            .unwrap_or(0.0);

        let current_speed = match d {
            MovementDirection::Idle => 0.0,
            MovementDirection::Back => -motion.vel.y,
            MovementDirection::Front => motion.vel.y,
            MovementDirection::Left => -motion.vel.x,
            MovementDirection::Right => motion.vel.x,
        };
        let mut speed_diff = desired_speed - current_speed;

        if speed_diff < 0.1 {
            speed_diff = 0.0;
        }

        let mut ratio = 0.0;
        if desired_speed != 0.0 {
            ratio = speed_diff / desired_speed;
        }
        if ratio < 0.0 {
            ratio = 0.0
        }
        let accel = travel_accel * ratio;

        let apply_accel = accel - movt.applied_accel;
        movt.applied_accel = accel;
        match d {
            MovementDirection::Front => {
                motion.accel.y += apply_accel
            }
            MovementDirection::Idle => {}
            MovementDirection::Back => {
                motion.accel.y -= apply_accel
            }
            MovementDirection::Left => {
                motion.accel.x -= apply_accel
            }
            MovementDirection::Right => {
                motion.accel.x += apply_accel
            }
        }
    }
}

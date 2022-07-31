use super::*;

pub fn get_camera(
    transform: Transform,
    tracking_target: Option<Entity>,
) -> (Transform, LooseTracking, Motion, Camera) {
    (
        transform,
        LooseTracking {
            target: tracking_target,
            radius: 100.0,
            inner_travel_vel: 1.0,
            inner_rotation_vel: 4.0,
            outer_rotation_vel: 0.5,
        },
        Motion::default(),
        Camera(),
    )
}

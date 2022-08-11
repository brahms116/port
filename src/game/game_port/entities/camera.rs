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
            perpendicular_ratio: 0.1,
            is_active: true,
        },
        Motion::default(),
        Camera(),
    )
}

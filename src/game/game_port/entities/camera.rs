use super::*;

pub fn get_camera(
    transform: Transform,
    parent: Parent,
) -> (Transform, Parent, Camera) {
    (transform, parent, Camera())
}

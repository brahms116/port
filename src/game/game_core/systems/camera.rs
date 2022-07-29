use super::*;

pub fn get_camera_transform<T: GameApi>(
    world: &World,
    api: &T,
) -> Option<Transform> {
    for (id, (_cam,)) in &mut world.query::<(&Camera,)>() {
        let transform = world
            .get::<Transform>(id)
            .ok()
            .map(|e| (*e).clone());

        if let Some(mut transform) = transform {
            transform = resolve_parent_transform(
                id, &transform, world, api,
            );
            return Some(transform);
        } else {
            return None;
        }
    }
    None
}

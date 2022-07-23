use super::*;

pub fn get_camera_transform(
    world: &World,
) -> Option<Transform> {
    for (id, (_cam,)) in &mut world.query::<(&Camera,)>() {
        return world
            .get::<Transform>(id)
            .ok()
            .map(|e| (*e).clone());
    }
    None
}

use super::*;
pub fn resolve_parent_transform<T: GameApi>(
    entity: Entity,
    transform: &Transform,
    world: &World,
    _api: &T,
) -> Transform {
    let mut cur_entity = entity;
    let mut cur_transform = transform.clone();

    while let Ok(parent) = world.get::<Parent>(cur_entity) {
        if parent.id.is_none() {
            break;
        }
        let parent = parent.id.unwrap();
        if let Ok(parent_transform) =
            world.get::<Transform>(parent)
        {
            cur_transform =
                cur_transform.apply(&parent_transform);
        }
        cur_entity = parent;
    }

    cur_transform
}

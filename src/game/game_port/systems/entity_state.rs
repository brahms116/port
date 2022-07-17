use super::*;
pub fn update_entity_state_system<
    T: 'static + Sync + Send,
>(
    world: &mut World,
) {
    for (_id, (state, cb)) in world
        .query_mut::<(&mut T, &UpdateStateCb<T>)>(
        )
    {
        cb.0(state)
    }
}

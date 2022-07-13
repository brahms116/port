use super::*;
use hecs::*;

/* Main loop */
pub fn main_loop<T: GameApi>(
    world: &mut World,
    api: &T,
) {
    /* Globals */
    let camera_transform =
        get_camera_transform(world);

    controller(world, api);

    /* Entity States */
    update_entity_state_system::<PlayerState>(
        world,
    );
    update_entity_state_system::<TrackState>(
        world,
    );

    /* Entity State Motion System */
    entity_motion_system::<PlayerState>(world);
    entity_motion_system::<TrackState>(world);

    /* Movement */
    motion_system(world);

    /* Collision */
    collision_system::<
        CollisionBoxMarker,
        PlayerState,
    >(world);

    /* Render System */

    render_static_system(
        world,
        api,
        &camera_transform,
    );

    render_system::<TrackState, T>(
        api,
        world,
        &camera_transform,
    );

    render_system::<PlayerState, T>(
        api,
        world,
        &camera_transform,
    );
}

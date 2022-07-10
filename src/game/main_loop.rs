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

    /* Entity States */
    update_entity_state_system::<PlayerState>(
        world,
    );

    /* Entity State Motion System */
    entity_motion_system::<PlayerState>(world);

    /* Movement */
    motion_system(world);

    /* Collision */
    collision_system(world);

    /* Render System */

    render_static_system(
        world,
        api,
        &camera_transform,
    );

    render_system::<PlayerState, T>(
        api,
        world,
        &camera_transform,
    );

    /* Game Controller */
    update_game_controller(world);
}

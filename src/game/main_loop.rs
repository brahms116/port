use super::*;
use hecs::*;

/* Main loop */
pub fn main_loop<T: GameApi>(world: &mut World, api: &T) {
    let inputs = system_input(world, api);

    system_player_movt(world, api, &inputs);
    system_loose_tracking(world, api);
    system_fade_animation_advance(world, api);
    system_fade_animation(world, api);
    system_squish_animation_advance(world, api);
    system_squish_animation(world, api);
    system_squish_movt(world, api);
    system_movement(world, api);
    system_rotation(world, api);
    system_motion(world, api);
    system_triangle_collision_box(world, api);
    system_static_collision(world, api);
    system_rectangle_render(world, api);
    system_triangle_render(world, api);
    system_render_ui(world, api);
    system_render(world, api);
}

use super::*;
use hecs::*;

/* Main loop */
pub fn main_loop<T: GameApi>(world: &mut World, api: &T) {
    system_fade_animation_advance(world, api);
    system_fade_animation(world, api);
    system_movement(world, api);
    system_motion(world, api);
    system_rectangle_render(world, api);
    system_render(world, api);
}

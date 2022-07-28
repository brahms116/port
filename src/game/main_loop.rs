use super::*;
use hecs::*;

/* Main loop */
pub fn main_loop<T: GameApi>(world: &mut World, api: &T) {
    api.log(&format!("{}", &api.inputs().is_down));
    let inputs = system_input(world, api);
    api.log(&format!("{:?}", &inputs));

    system_fade_animation_advance(world, api);
    system_fade_animation(world, api);
    system_squish_animation_advance(world, api);
    system_squish_animation(world, api);
    system_squish_movt(world, api);
    system_movement(world, api);
    system_motion(world, api);
    system_rectangle_render(world, api);
    system_render(world, api);
}

use super::*;
use hecs::*;

/* Main loop */
pub fn main_loop<T: GameApi>(world: &mut World, api: &T) {
    system_render(world, api);
}

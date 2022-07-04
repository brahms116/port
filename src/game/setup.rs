use super::game_components::*;
use hecs::*;
pub fn setup(world: &mut World) {
    world.spawn((FrameCount(0), StageCount(0)));
}

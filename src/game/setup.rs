use super::entity::*;
use super::game_components::*;
use hecs::*;
pub fn setup(world: &mut World) {
    world.spawn((GameController(), FrameCount(0), StageCount(0)));
    world.spawn(basic_rect());
}

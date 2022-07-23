use super::*;
use hecs::*;
pub fn setup(world: &mut World) {
    world.spawn(create_player_square(Transform::new(
        Vec2::new(-600.0, -134.0),
        0.0,
    )));
}

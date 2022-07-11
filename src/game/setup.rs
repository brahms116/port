use super::*;
use hecs::*;
pub fn setup(world: &mut World) {
    world.spawn((
        GameController(),
        FrameCount(0),
        StageCount(0),
    ));
    world.spawn(player_square(
        Transform::new(Vec2::default(), 45.0),
        Motion::default(),
    ));
    world.spawn(collision_box());
}

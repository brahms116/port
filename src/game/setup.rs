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
        PlayerState::motion(),
    ));
    world.spawn(collision_box(Transform::new(
        Vec2::new(-200.0, 200.0),
        0.0,
    )));
    world.spawn(collision_box(Transform::new(
        Vec2::new(-400.0, 200.0),
        0.0,
    )));
    world.spawn(player_square(
        Transform::new(
            Vec2::new(-400.0, 0.0),
            0.0,
        ),
        Motion::default(),
        PlayerState::motion(),
    ));
    world.spawn(collision_box(Transform::new(
        Vec2::new(400.0, -200.0),
        0.0,
    )));
    world.spawn(player_square(
        Transform::new(
            Vec2::new(400.0, -184.0),
            0.0,
        ),
        Motion::default(),
        PlayerState::jump(),
    ));
}

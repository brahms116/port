use super::*;
use hecs::*;
pub fn setup(world: &mut World) {
    world.spawn((InputController::new(),));
    let player = world.spawn(create_player_square(
        Transform::new(Vec2::new(0.0, 0.0), 0.0),
    ));
    world.spawn(get_rectangle(
        400.0,
        24.0,
        Transform::new(Vec2::new(100.0, 0.0), 0.0),
    ));
    world.spawn(get_rectangle(
        400.0,
        24.0,
        Transform::new(Vec2::new(-100.0, 0.0), 0.0),
    ));
    world.spawn(get_rectangle(
        24.0,
        500.0,
        Transform::new(Vec2::new(-100.0, 500.0), -50.0),
    ));
    world.spawn(get_camera(
        Transform::default(),
        Parent::new(Some(player)),
        // Parent::new(None),
    ));
}

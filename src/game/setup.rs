use super::*;
use hecs::*;
pub fn setup(world: &mut World) {
    world.spawn(get_controller());
    world.spawn(collision_box(Transform::new(
        Vec2::new(-600.0, -150.0),
        0.0,
    )));
    world.spawn(player_square(
        Transform::new(
            Vec2::new(-600.0, -134.0),
            0.0,
        ),
        Motion::default(),
        PlayerState::jump(),
    ));
}

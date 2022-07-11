use super::*;
use hecs::*;
pub fn setup(world: &mut World) {
    world.spawn((
        GameController(),
        FrameCount(0),
        StageCount(0),
    ));
    world.spawn(player_square(
        Transform::new(
            Vec2::new(100.0, 0.0),
            0.0,
        ),
        Motion {
            accel: Vec2::default(),
            angular_accel: 0.0,
            vel: Vec2::default(),
            angular_vel: 2.0,
        },
    ));
    world.spawn(player_square(
        Transform::new(
            Vec2::new(120.0, 0.0),
            0.0,
        ),
        Motion {
            accel: Vec2::default(),
            angular_accel: 0.0,
            vel: Vec2::default(),
            angular_vel: 2.4,
        },
    ));
    world.spawn(player_square(
        Transform::new(
            Vec2::new(140.0, 0.0),
            180.0,
        ),
        Motion {
            accel: Vec2::default(),
            angular_accel: 0.0,
            vel: Vec2::default(),
            angular_vel: -2.6,
        },
    ));
    //    world.spawn(collision_box());
}

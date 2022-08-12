use super::*;
use hecs::*;

fn set_walls(
    world: &mut World,
    points: Vec<Vec2>,
    p0: Option<Vec2>,
    pn: Option<Vec2>,
) {
    for (i, v) in points.iter().enumerate() {
        let len = points.len();
        if i < len - 1 {
            world.spawn(get_wall(
                *points.get(i).unwrap(),
                *points.get(i + 1).unwrap(),
                points.get(i - 1).map(|e| *e),
                points.get(i + 2).map(|e| *e),
            ));
        }
    }
}

pub fn setup(world: &mut World) {
    /* Start Hub */

    let x = 0.0;
    let y = 0.0;
    set_walls(
        world,
        vec![
            Vec2::new(x + 599.5, y + 1218.5),
            Vec2::new(x + -79.0, y + 773.5),
            Vec2::new(x + -79.0, y + 359.0),
            Vec2::new(x + -226.0, y + 208.0),
            Vec2::new(x + -226.0, y + -24.5),
            Vec2::new(x + -79.0, y + -177.0),
            Vec2::new(x + 87.5, y + -177.0),
            Vec2::new(x + 255.0, y + -24.5),
            Vec2::new(x + 240.5, y + 190.0),
            Vec2::new(x + 87.5, y + 359.0),
            Vec2::new(x + 137.0, y + 655.5),
            Vec2::new(x + 687.5, y + 1031.0),
        ],
        None,
        None,
    );

    world.spawn((InputController::new(),));
    let player = world.spawn(create_player_square(
        Transform::new(Vec2::new(0.0, 0.0), 0.0),
    ));

    world.spawn(get_element(
        String::from("hello-there"),
        Transform::new(Vec2::new(-80.0, 100.0), 0.0),
    ));

    world.spawn(get_element(
        String::from("instructions"),
        Transform::new(Vec2::new(-80.0, 45.0), 0.0),
    ));

    world.spawn(get_element(
        String::from("intro"),
        Transform::new(Vec2::new(281.0, 947.0), -57.0),
    ));

    world.spawn(get_camera(
        Transform::new(Vec2::new(0.0, 0.0), 0.0),
        Some(player),
    ));
}

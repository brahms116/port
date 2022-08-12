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

    set_walls(
        world,
        vec![
            Vec2::new(1300.0, 1700.0),
            Vec2::new(900.0, 1500.0),
            Vec2::new(850.0, 1350.0),
            Vec2::new(650.0, 950.0),
            Vec2::new(100.0, 950.0),
            Vec2::new(-100.0, 750.0),
            Vec2::new(-100.0, 500.0),
            Vec2::new(-350.0, 0.0),
            Vec2::new(-100.0, -500.0),
            Vec2::new(100.0, -500.0),
            Vec2::new(350.0, 0.0),
            Vec2::new(100.0, 500.0),
            Vec2::new(215.0, 690.0),
            Vec2::new(400.0, 750.0),
            Vec2::new(650.0, 750.0),
            Vec2::new(850.0, 350.0),
            Vec2::new(900.0, 100.0),
            Vec2::new(1300.0, -100.0),
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
        Transform::new(Vec2::new(550.0, 900.0), -90.0),
    ));

    world.spawn(get_element(
        String::from("title"),
        Transform::new(Vec2::new(1400.0, 1000.0), -90.0),
    ));

    world.spawn(get_camera(
        Transform::new(Vec2::new(0.0, 0.0), 0.0),
        Some(player),
    ));
}

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
            Vec2::new(-400.0, 400.0),
            Vec2::new(-200.0, 150.0),
            Vec2::new(-250.0, -0.0),
            Vec2::new(-200.0, -100.0),
            /* Origin point */
            Vec2::new(0.0, -200.0),
            Vec2::new(200.0, -100.0),
            Vec2::new(300.0, 100.0),
            Vec2::new(200.0, 400.0),
            Vec2::new(-200.0, 500.0),
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
    world.spawn(get_camera(
        Transform::new(Vec2::new(0.0, 0.0), 0.0),
        Some(player),
    ));
}

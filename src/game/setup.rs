use super::*;
use hecs::*;

pub fn setup(world: &mut World) {
    /* Start Hub */

    let walls = get_octagon(
        Vec2::y() * 25.0,
        600.0,
        Some(100.0),
        None,
        None,
        None,
    );

    world.spawn_batch(walls);

    /* Title Area */
    let walls = get_octagon(
        Vec2::y() * 925.0,
        800.0,
        Some(100.0),
        Some(100.0),
        Some(100.0),
        Some(100.0),
    );
    world.spawn_batch(walls);

    /* Project 1 */
    let walls = get_octagon(
        Vec2::new(-1100.0, 925.0),
        1000.0,
        None,
        Some(100.0),
        None,
        Some(100.0),
    );
    world.spawn_batch(walls);

    world.spawn(get_element(
        String::from("port-project"),
        Transform::new(Vec2::new(-1175.0, 625.0), 90.0),
    ));

    /* join 1-2 */
    let walls = set_path(
        vec![
            Vec2::new(-1700.0, 925.0),
            Vec2::new(-1800.0, 925.0),
            Vec2::new(-2200.0, 1325.0),
            Vec2::new(-2200.0, 1425.0),
        ],
        None,
        None,
    );

    world.spawn_batch(walls);

    /* project 2 */
    let walls = get_octagon(
        Vec2::new(-2200.0, 2025.0),
        1000.0,
        Some(100.0),
        None,
        Some(100.0),
        None,
    );

    world.spawn(get_element(
        String::from("memble-project"),
        Transform::new(Vec2::new(-2500.0, 2400.0), 0.0),
    ));

    world.spawn_batch(walls);

    /* join 2-3 */

    let walls = set_path(
        vec![
            Vec2::new(-2200.0, 2625.0),
            Vec2::new(-2200.0, 2725.0),
            Vec2::new(-1800.0, 3125.0),
            Vec2::new(-1700.0, 3125.0),
        ],
        None,
        None,
    );
    world.spawn_batch(walls);

    /* project 3 */
    let walls = get_octagon(
        Vec2::new(-1100.0, 3125.0),
        1000.0,
        None,
        Some(100.0),
        None,
        Some(100.0),
    );

    world.spawn_batch(walls);

    // world.spawn(get_element(
    //     String::from("memble-project"),
    //     Transform::new(Vec2::new(-875.0, 3425.0), -90.0),
    // ));

    /* join 3-4 */
    let walls = set_path(
        vec![
            Vec2::new(-500.0, 3125.0),
            Vec2::new(-400.0, 3125.0),
            Vec2::new(0.0, 2725.0),
            Vec2::new(0.0, 2625.0),
        ],
        None,
        None,
    );
    world.spawn_batch(walls);

    /* project 4 */
    let walls = get_octagon(
        Vec2::y() * 2025.0,
        1000.0,
        Some(100.0),
        None,
        Some(100.0),
        None,
    );
    world.spawn_batch(walls);

    world.spawn((InputController::new(),));
    let player = world.spawn(create_player_square(
        Transform::new(Vec2::new(0.0, 0.0), 0.0),
    ));

    world.spawn(get_element(
        String::from("hello-there"),
        Transform::new(Vec2::new(-90.0, 100.0), 0.0),
    ));

    world.spawn(get_element(
        String::from("instructions"),
        Transform::new(Vec2::new(-90.0, 45.0), 0.0),
    ));

    world.spawn(get_element(
        String::from("intro"),
        Transform::new(Vec2::new(550.0, 900.0), -90.0),
    ));

    world.spawn(get_element(
        String::from("title"),
        Transform::new(Vec2::new(-270.0, 1200.0), 0.0),
    ));

    world.spawn(get_camera(
        Transform::new(Vec2::new(0.0, 0.0), 0.0),
        Some(player),
    ));
}

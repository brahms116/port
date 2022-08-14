use super::*;
use hecs::*;

const PATH_WIDTH: f64 = 200.0;

fn get_octagon(
    world: &mut World,
    center: Vec2,
    length: f64,
    top: Option<f64>,
    right: Option<f64>,
    bottom: Option<f64>,
    left: Option<f64>,
) {
    let mut result: Vec<Vec2> = vec![];

    let hex_const =
        1.0 + 2.0 * (std::f64::consts::FRAC_PI_4.sin());

    let start = center
        + Vec2::new(
            -length / (2.0 * hex_const),
            length / 2.0,
        );

    let end = center
        + Vec2::new(
            -length / 2.0,
            length / (2.0 * hex_const),
        );

    let mut has_end_used: bool = false;

    /* top */

    result.push(
        center
            + Vec2::new(
                -length / (2.0 * hex_const),
                length / 2.0,
            ),
    );

    if let Some(l) = top {
        result.push(
            center
                + Vec2::new(
                    -PATH_WIDTH / 2.0,
                    length / 2.0,
                ),
        );
        result.push(
            center
                + Vec2::new(
                    -PATH_WIDTH / 2.0,
                    length / 2.0 + l,
                ),
        );

        set_walls(world, result, Some(end), None);
        result = vec![];
        has_end_used = true;

        result.push(
            center
                + Vec2::new(
                    PATH_WIDTH / 2.0,
                    length / 2.0 + l,
                ),
        );

        result.push(
            center
                + Vec2::new(PATH_WIDTH / 2.0, length / 2.0),
        );
    }

    result.push(
        center
            + Vec2::new(
                length / (2.0 * hex_const),
                length / 2.0,
            ),
    );

    /* right */

    result.push(
        center
            + Vec2::new(
                length / 2.0,
                length / (2.0 * hex_const),
            ),
    );

    if let Some(l) = right {
        result.push(
            center
                + Vec2::new(length / 2.0, PATH_WIDTH / 2.0),
        );
        result.push(
            center
                + Vec2::new(
                    length / 2.0 + l,
                    PATH_WIDTH / 2.0,
                ),
        );

        let prev =
            if has_end_used { None } else { Some(end) };

        if prev.is_some() {
            has_end_used = true;
        }

        set_walls(world, result, prev, None);
        result = vec![];

        result.push(
            center
                + Vec2::new(
                    length / 2.0 + l,
                    -PATH_WIDTH / 2.0,
                ),
        );

        result.push(
            center
                + Vec2::new(
                    length / 2.0,
                    -PATH_WIDTH / 2.0,
                ),
        )
    }

    result.push(
        center
            + Vec2::new(
                length / 2.0,
                -length / (2.0 * hex_const),
            ),
    );

    /* bottom */

    result.push(
        center
            + Vec2::new(
                length / (2.0 * hex_const),
                -length / 2.0,
            ),
    );

    if let Some(l) = bottom {
        result.push(
            center
                + Vec2::new(
                    PATH_WIDTH / 2.0,
                    -length / 2.0,
                ),
        );

        result.push(
            center
                + Vec2::new(
                    PATH_WIDTH / 2.0,
                    -length / 2.0 - l,
                ),
        );

        let prev =
            if has_end_used { None } else { Some(end) };

        if prev.is_some() {
            has_end_used = true;
        }

        set_walls(world, result, prev, None);
        result = vec![];

        result.push(
            center
                + Vec2::new(
                    -PATH_WIDTH / 2.0,
                    -length / 2.0 - l,
                ),
        );

        result.push(
            center
                + Vec2::new(
                    -PATH_WIDTH / 2.0,
                    -length / 2.0,
                ),
        );
    }

    result.push(
        center
            + Vec2::new(
                -length / (2.0 * hex_const),
                -length / 2.0,
            ),
    );

    /* left */
    result.push(
        center
            + Vec2::new(
                -length / 2.0,
                -length / (2.0 * hex_const),
            ),
    );

    if let Some(l) = left {
        result.push(
            center
                + Vec2::new(
                    -length / 2.0,
                    -PATH_WIDTH / 2.0,
                ),
        );

        result.push(
            center
                + Vec2::new(
                    -length / 2.0 - l,
                    -PATH_WIDTH / 2.0,
                ),
        );
        let prev =
            if has_end_used { None } else { Some(end) };

        if prev.is_some() {
            has_end_used = true;
        }

        set_walls(world, result, prev, None);
        result = vec![];

        result.push(
            center
                + Vec2::new(
                    -length / 2.0 - l,
                    PATH_WIDTH / 2.0,
                ),
        );

        result.push(
            center
                + Vec2::new(
                    -length / 2.0,
                    PATH_WIDTH / 2.0,
                ),
        )
    }

    result.push(
        center
            + Vec2::new(
                -length / 2.0,
                length / (2.0 * hex_const),
            ),
    );

    result.push(start);
    let prev = if has_end_used { None } else { Some(end) };
    set_walls(world, result, prev, Some(start + Vec2::x()));
}

fn set_walls(
    world: &mut World,
    points: Vec<Vec2>,
    p0: Option<Vec2>,
    pn: Option<Vec2>,
) {
    for i in 0..points.len() {
        let len = points.len();

        let mut prev = points.get(i - 1).map(|e| *e);
        let mut nxt = points.get(i + 2).map(|e| *e);

        if i == 0 {
            prev = p0;
        }

        if i == len - 2 {
            nxt = pn;
        }

        if i < len - 1 {
            world.spawn(get_wall(
                *points.get(i).unwrap(),
                *points.get(i + 1).unwrap(),
                prev,
                nxt,
            ));
        }
    }
}

pub fn setup(world: &mut World) {
    /* Start Hub */

    get_octagon(
        world,
        Vec2::y() * 25.0,
        600.0,
        Some(100.0),
        None,
        None,
        None,
    );

    /* Title Area */
    get_octagon(
        world,
        Vec2::y() * 825.0,
        800.0,
        Some(100.0),
        Some(100.0),
        Some(100.0),
        Some(100.0),
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
        Transform::new(Vec2::new(-270.0, 1100.0), 0.0),
    ));

    world.spawn(get_camera(
        Transform::new(Vec2::new(0.0, 0.0), 0.0),
        Some(player),
    ));
}

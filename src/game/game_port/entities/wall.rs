use super::*;

const WALL_WIDTH: f64 = 5.0;
const PATH_WIDTH: f64 = 200.0;

type Wall =
    (Transform, Render, CollisionBox, StaticCollider);

pub fn get_wall(
    p1: Vec2,
    p2: Vec2,
    p0: Option<Vec2>,
    p3: Option<Vec2>,
) -> Wall {
    let (v1, v2, v3, v4) =
        join_rect(p1, p2, p0, p3, WALL_WIDTH);

    let surface = Surface {
        points: vec![v1, v2, v3, v4],
        color: RGBA {
            r: 67,
            g: 78,
            b: 66,
            a: 1.0,
        },
    };

    (
        Transform::default(),
        Render(vec![surface]),
        CollisionBox::new(vec![v1, v2, v3, v4]),
        StaticCollider(),
    )
}

pub fn get_octagon(
    center: Vec2,
    length: f64,
    top: Option<f64>,
    right: Option<f64>,
    bottom: Option<f64>,
    left: Option<f64>,
) -> Vec<Wall> {
    let mut queue: Vec<Vec2> = vec![];

    let mut result: Vec<Wall> = vec![];

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

    queue.push(
        center
            + Vec2::new(
                -length / (2.0 * hex_const),
                length / 2.0,
            ),
    );

    if let Some(l) = top {
        queue.push(
            center
                + Vec2::new(
                    -PATH_WIDTH / 2.0,
                    length / 2.0,
                ),
        );
        queue.push(
            center
                + Vec2::new(
                    -PATH_WIDTH / 2.0,
                    length / 2.0 + l,
                ),
        );

        result.append(&mut set_walls(
            queue,
            Some(end),
            None,
        ));
        queue = vec![];
        has_end_used = true;

        queue.push(
            center
                + Vec2::new(
                    PATH_WIDTH / 2.0,
                    length / 2.0 + l,
                ),
        );

        queue.push(
            center
                + Vec2::new(PATH_WIDTH / 2.0, length / 2.0),
        );
    }

    queue.push(
        center
            + Vec2::new(
                length / (2.0 * hex_const),
                length / 2.0,
            ),
    );

    /* right */

    queue.push(
        center
            + Vec2::new(
                length / 2.0,
                length / (2.0 * hex_const),
            ),
    );

    if let Some(l) = right {
        queue.push(
            center
                + Vec2::new(length / 2.0, PATH_WIDTH / 2.0),
        );
        queue.push(
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

        result.append(&mut set_walls(queue, prev, None));
        queue = vec![];

        queue.push(
            center
                + Vec2::new(
                    length / 2.0 + l,
                    -PATH_WIDTH / 2.0,
                ),
        );

        queue.push(
            center
                + Vec2::new(
                    length / 2.0,
                    -PATH_WIDTH / 2.0,
                ),
        )
    }

    queue.push(
        center
            + Vec2::new(
                length / 2.0,
                -length / (2.0 * hex_const),
            ),
    );

    /* bottom */

    queue.push(
        center
            + Vec2::new(
                length / (2.0 * hex_const),
                -length / 2.0,
            ),
    );

    if let Some(l) = bottom {
        queue.push(
            center
                + Vec2::new(
                    PATH_WIDTH / 2.0,
                    -length / 2.0,
                ),
        );

        queue.push(
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

        result.append(&mut set_walls(queue, prev, None));
        queue = vec![];

        queue.push(
            center
                + Vec2::new(
                    -PATH_WIDTH / 2.0,
                    -length / 2.0 - l,
                ),
        );

        queue.push(
            center
                + Vec2::new(
                    -PATH_WIDTH / 2.0,
                    -length / 2.0,
                ),
        );
    }

    queue.push(
        center
            + Vec2::new(
                -length / (2.0 * hex_const),
                -length / 2.0,
            ),
    );

    /* left */
    queue.push(
        center
            + Vec2::new(
                -length / 2.0,
                -length / (2.0 * hex_const),
            ),
    );

    if let Some(l) = left {
        queue.push(
            center
                + Vec2::new(
                    -length / 2.0,
                    -PATH_WIDTH / 2.0,
                ),
        );

        queue.push(
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

        result.append(&mut set_walls(queue, prev, None));
        queue = vec![];

        queue.push(
            center
                + Vec2::new(
                    -length / 2.0 - l,
                    PATH_WIDTH / 2.0,
                ),
        );

        queue.push(
            center
                + Vec2::new(
                    -length / 2.0,
                    PATH_WIDTH / 2.0,
                ),
        )
    }

    queue.push(
        center
            + Vec2::new(
                -length / 2.0,
                length / (2.0 * hex_const),
            ),
    );

    queue.push(start);
    let prev = if has_end_used { None } else { Some(end) };
    result.append(&mut set_walls(
        queue,
        prev,
        Some(start + Vec2::x()),
    ));
    result
}

pub fn set_walls(
    points: Vec<Vec2>,
    p0: Option<Vec2>,
    pn: Option<Vec2>,
) -> Vec<Wall> {
    let mut result: Vec<Wall> = vec![];
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
            result.push(get_wall(
                *points.get(i).unwrap(),
                *points.get(i + 1).unwrap(),
                prev,
                nxt,
            ));
        }
    }
    result
}

const BEND_DISTANCE: f64 = 300.0;

pub fn set_path(
    points: Vec<Vec2>,
    p0: Option<Vec2>,
    pn: Option<Vec2>,
) -> Vec<Wall> {
    let mut top: Vec<Vec2> = vec![];
    let mut bottom: Vec<Vec2> = vec![];

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
            let (v1, v2, v3, v4) = join_rect(
                points[i],
                points[i + 1],
                prev,
                nxt,
                PATH_WIDTH,
            );

            if i == 0 {
                top.push(v4);
                top.push(v3);
                bottom.push(v1);
                bottom.push(v2);
            } else {
                let diff1 =
                    (v4 - top.last().unwrap()).mag();
                let diff2 =
                    (v4 - bottom.last().unwrap()).mag();

                if diff1 < diff2 {
                    top.push(v3);
                    bottom.push(v2);
                } else {
                    bottom.push(v3);
                    top.push(v2);
                }
            }
        }
    }

    let mut top_start: Option<Vec2> = None;
    let mut top_end: Option<Vec2> = None;
    let mut bottom_start: Option<Vec2> = None;
    let mut bottom_end: Option<Vec2> = None;

    if points.len() > 0 {
        if let Some(v) = p0 {
            let diff = points[0] - v;

            let offset =
                diff.perpendicular() * (WALL_WIDTH / 2.0);

            let v1 = v + offset;
            let v2 = v - offset;

            let diff1 = (top[0] - v1).mag();
            let diff2 = (bottom[0] - v1).mag();

            if diff1 < diff2 {
                top_start = Some(v1);
                bottom_start = Some(v2);
            } else {
                top_start = Some(v2);
                bottom_start = Some(v1);
            }
        }

        if let Some(v) = pn {
            let diff = v - points.last().unwrap();

            let offset =
                diff.perpendicular() * (WALL_WIDTH / 2.0);

            let v1 = v + offset;
            let v2 = v - offset;

            let diff1 = (v1 - top.last().unwrap()).mag();
            let diff2 = (v1 - bottom.last().unwrap()).mag();

            if diff1 < diff2 {
                top_end = Some(v1);
                bottom_end = Some(v2);
            } else {
                top_end = Some(v2);
                bottom_end = Some(v1);
            }
        }
    }

    // web_sys::console::log_1(&format!("{:?}", top).into());
    let mut walls_top = set_walls(top, top_start, top_end);
    let mut walls_bottom =
        set_walls(bottom, bottom_start, bottom_end);

    walls_top.append(&mut walls_bottom);

    walls_top
}

// pub fn get_bend_1(corner: Vec2) -> Vec<Wall> {
//     let offset =
//         PATH_WIDTH * std::f64::consts::FRAC_PI_8.tan();
//     let v1 = Vec2::new(-BEND_DISTANCE, 0.0);
//     let v2 = Vec2::new(0.0, -BEND_DISTANCE);

//     let wall1 =
//         get_wall(corner + v1, corner + v2, None, None);

//     let v3 = Vec2::new(-BEND_DISTANCE - offset, 0.0)
//         + Vec2::new(0.0, -PATH_WIDTH);

//     let v4 = Vec2::new(0.0, -BEND_DISTANCE - offset)
//         + Vec2::new(-PATH_WIDTH, 0.0);

//     let wall2 =
//         get_wall(corner + v3, corner + v4, None, None);

//     vec![wall1, wall2]
// }

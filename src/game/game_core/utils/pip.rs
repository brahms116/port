use super::*;

pub fn pip(point: Vec2, poly: &Vec<Vec2>) -> bool {
    let len = poly.len();
    let j = len - 1;

    let mut crossed = 0;

    for i in 0..len {
        let ivec = poly[i];
        let jvec = poly[j];

        if point.x == ivec.y && point.y == ivec.y {
            return true;
        }
        if (point.y > ivec.y) != (point.y > jvec.y) {
            if ivec.x == jvec.x {
                if point.x == ivec.x {
                    /* on boundary */
                    return true;
                } else if point.x < ivec.x {
                    crossed += 1;
                }
            } else {
                let slope =
                    (ivec.y - jvec.y) / (ivec.x - jvec.x);

                let higher_y_point = if jvec.y > ivec.y {
                    jvec
                } else {
                    ivec
                };

                if point.x == higher_y_point.x {
                    if slope < 0.0 {
                        crossed += 1;
                    }
                } else {
                    let slope2 = (higher_y_point.y
                        - point.y)
                        / (higher_y_point.x - point.x);
                    if slope2 == slope {
                        return true;
                    }

                    if slope2 < slope && slope > 0.0 {
                        crossed += 1;
                    } else if slope2 > slope && slope < 0.0
                    {
                        crossed += 1;
                    }
                }
            }
        }
    }
    crossed % 2 > 0
}

#[test]
fn pip_should_be_correct() {
    let poly: Vec<Vec2> = vec![
        Vec2::new(-1.0, 0.0),
        Vec2::new(0.0, 2.0),
        Vec2::new(1.0, 0.0),
    ];
    assert_eq!(true, pip(Vec2::new(0.0, 0.5), &poly));
    assert_eq!(false, pip(Vec2::new(0.0, -0.5), &poly));
}

pub fn polygon_collision(
    poly1: &Vec<Vec2>,
    poly2: &Vec<Vec2>,
) -> bool {
    for x in poly1.iter() {
        if pip(*x, poly2) {
            return true;
        }
    }

    for x in poly2.iter() {
        if pip(*x, poly1) {
            return true;
        }
    }
    false
}

use super::*;

pub fn pip(point: Vec2, poly: &Vec<Vec2>) -> bool {
    let len = poly.len();
    let mut j = len - 1;

    let mut crossed = 0;

    for i in 0..len {
        let ivec = poly[i];
        let jvec = poly[j];
        println!(
            "ivec, jvec, point: {:?} {:?} {:?}",
            ivec, jvec, point
        );

        if point.x == ivec.y && point.y == ivec.y {
            println!(
                "exact same point {:?} {:?}",
                point, ivec
            );
            return true;
        }
        if (point.y > ivec.y) != (point.y > jvec.y) {
            if ivec.x == jvec.x {
                if point.x == ivec.x {
                    /* on boundary */
                    println!(
                        "on boundary straight y! {} {}",
                        point.x, ivec.x
                    );
                    return true;
                } else if point.x < ivec.x {
                    println!(
                        "boundary straight y and point is on left! {} {}",
                        point.x, ivec.x
                    );
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

                println!("slope {}", slope);

                if point.x == higher_y_point.x {
                    if slope < 0.0 {
                        println!(
                            "point is straight under {}",
                            slope
                        );
                        crossed += 1;
                    }
                } else {
                    let slope2 = (higher_y_point.y
                        - point.y)
                        / (higher_y_point.x - point.x);
                    println!("slope2 {}", slope2);
                    if slope2 == slope {
                        println!(
                            "boundary straight slope ! {} {}",
                            slope,slope2 
                        );
                        return true;
                    }

                    if slope2 > 0.0 {
                        if slope > 0.0 && slope > slope2 {
                            println!("both positive",);
                            crossed += 1;
                        } else if slope < 0.0 {
                            println!("slope2 positive, slope negative",);
                            crossed += 1;
                        }
                    } else if slope2 < 0.0 {
                        if slope > slope2 && slope < 0.0 {
                            println!("both negative",);
                            crossed += 1;
                        }
                    }
                }
            }
        }
        j = i;
    }
    println!("{}", crossed);
    crossed % 2 > 0
}

#[test]
fn pip_should_be_correct() {
    let poly: Vec<Vec2> = vec![
        Vec2 {
            x: -244.60165436233183,
            y: 0.3836486121626088,
        },
        Vec2 {
            x: -196.27322003750035,
            y: -96.27322003750035,
        },
        Vec2 {
            x: -203.72677996249965,
            y: -103.72677996249965,
        },
        Vec2 {
            x: -255.39834563766817,
            y: -0.3836486121626088,
        },
    ];
    assert_eq!(
        false,
        pip(Vec2::new(-13.2, 0.12908000000000053), &poly)
    );
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

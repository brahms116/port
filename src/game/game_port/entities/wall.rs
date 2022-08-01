use super::*;

const WALL_WIDTH: f64 = 5.0;

pub fn get_wall(
    p1: Vec2,
    p2: Vec2,
    p0: Option<Vec2>,
    p3: Option<Vec2>,
) {
    let diff = p2 - p1;

    //start with the first end
    // 1. find the 3 points of the p1
    let offset = diff.perpendicular() * WALL_WIDTH;

    let mut v1 = p1 + offset;
    let mut v4 = p1 - offset;

    let mut v2 = p2 + offset;
    let mut v4 = p2 - offset;

    if let Some(vec) = p0 {
        let prev = vec - p1;
        let angle = prev.angle(diff);
        let hyp = 5.0
            / (std::f64::consts::FRAC_PI_2 - (angle / 2.0))
                .cos();
    }
}

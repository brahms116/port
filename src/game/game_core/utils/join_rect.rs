use super::*;

pub fn join_rect(
    p1: Vec2,
    p2: Vec2,
    p0: Option<Vec2>,
    p3: Option<Vec2>,
    width: f64,
) -> (Vec2, Vec2, Vec2, Vec2) {
    let diff = p2 - p1;

    let half_width = width / 2.0;

    let offset = diff.perpendicular() * half_width;

    let mut v1 = p1 + offset;
    let mut v4 = p1 - offset;

    let mut v2 = p2 + offset;
    let mut v3 = p2 - offset;

    if let Some(vec) = p0 {
        let prev = vec - p1;
        let angle = prev.angle(diff);
        let hyp = half_width
            / (std::f64::consts::FRAC_PI_2 - (angle / 2.0))
                .cos();

        let unit = (prev.unit() + diff.unit()).unit();

        let correct_vec = unit * hyp;
        let diff1 = (offset - unit).mag();
        let diff2 = (-offset - unit).mag();
        if diff1 < diff2 {
            v1 = p1 + correct_vec;
            v4 = p1 - correct_vec;
        } else {
            v4 = p1 + correct_vec;
            v1 = p1 - correct_vec;
        }
    }

    if let Some(vec) = p3 {
        let prev = vec - p2;
        let angle = prev.angle(-diff);
        let hyp = half_width
            / (std::f64::consts::FRAC_PI_2 - (angle / 2.0))
                .cos();

        let unit = (prev.unit() - diff.unit()).unit();
        let correct_vec = unit * hyp;
        let diff1 = (offset - unit).mag();
        let diff2 = (-offset - unit).mag();
        if diff1 < diff2 {
            v2 = p2 + correct_vec;
            v3 = p2 - correct_vec;
        } else {
            v3 = p2 + correct_vec;
            v2 = p2 - correct_vec;
        }
    }

    (v1, v2, v3, v4)
}

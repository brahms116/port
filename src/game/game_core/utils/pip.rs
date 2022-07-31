use super::*;

pub fn pip(point: Vec2, poly: Vec<Vec2>) -> bool {
    let len = poly.len();
    let j = len - 1;

    for i in 0..len {
        let ivec = poly[i];
        let jvec = poly[j];
        if (point.y > ivec.y) != (point.y > jvec.y) {}
    }

    false
}

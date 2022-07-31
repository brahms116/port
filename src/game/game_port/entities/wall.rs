use super::*;

const WALL_WIDTH: f64 = 5.0;

pub fn get_wall(
    p1: Vec2,
    p2: Vec2,
    p0: Option<Vec2>,
    p3: Option<Vec2>,
) {
    let diff = p2 - p1;
    let midway = p1 + diff;
}

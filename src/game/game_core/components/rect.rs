use super::*;

#[derive(Clone, Debug)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
    pub transform: Transform,
}

impl Rect {
    pub fn two_point_flat(x1y1: Vec2, x2y2: Vec2) -> Self {
        let width = x2y2.x - x1y1.x;
        let height = x2y2.y - x1y1.y;

        Self {
            height,
            width,
            transform: Transform::new(
                Vec2::new(
                    width / 2.0 + x1y1.x,
                    height / 2.0 + x1y1.y,
                ),
                0.0,
            ),
        }
    }
    pub fn corners(&self) -> (Vec2, Vec2, Vec2, Vec2) {
        let x = self.width / 2.0;
        let y = self.height / 2.0;

        (
            Vec2::new(-x, -y).apply(&self.transform),
            Vec2::new(-x, y).apply(&self.transform),
            Vec2::new(x, y).apply(&self.transform),
            Vec2::new(x, -y).apply(&self.transform),
        )
    }
    pub fn check_point_in_rect(&self, point: Vec2) -> bool {
        let diff = point - self.transform.position;

        let diff = diff
            .rotate_deg(360.0 - self.transform.rotation);

        let x_diff = diff.x.abs();
        let y_diff = diff.y.abs();

        self.width / 2.0 >= x_diff
            && self.height / 2.0 >= y_diff
    }
    pub fn check_collision(
        static_rect: &Rect,
        moving_rect: &Rect,
        vel_vec: Vec2,
    ) -> Option<Vec2> {
        let mut static_vec: Option<Vec2> = None;
        let mut moving_vec: Option<Vec2> = None;

        let (v1, v2, v3, v4) = moving_rect.corners();
        for i in [v1, v2, v3, v4] {
            if static_rect.check_point_in_rect(i) {
                let mut checkee = -vel_vec;
                let mut offset = Vec2::default();
                let mut result: Option<Vec2> = None;
                for _ in 0..9 {
                    let new_vec = checkee * 0.5;

                    let new_point = new_vec + offset + i;
                    if static_rect
                        .check_point_in_rect(new_point)
                    {
                        offset += new_vec;
                        checkee = new_vec;
                    } else {
                        result = Some(new_vec + offset);
                        checkee = new_vec;
                    }
                    static_vec =
                        Some(result.unwrap_or(-vel_vec));
                }
            }
        }
        let (v1, v2, v3, v4) = static_rect.corners();
        for i in [v1, v2, v3, v4] {
            if moving_rect.check_point_in_rect(i) {
                let mut checkee = -vel_vec;
                let mut offset = Vec2::default();
                let mut result: Option<Vec2> = None;
                for _ in 0..9 {
                    let new_vec = checkee * 0.5;
                    let mut new_rect = moving_rect.clone();
                    new_rect.transform.position +=
                        new_vec + offset;

                    if new_rect.check_point_in_rect(i) {
                        offset += new_vec;
                        checkee = new_vec;
                    } else {
                        result = Some(new_vec + offset);
                        checkee = new_vec;
                    }
                }
                moving_vec =
                    Some(result.unwrap_or(-vel_vec))
            }
        }

        match (static_vec, moving_vec) {
            (Some(n), Some(v)) => {
                if n.mag() > v.mag() {
                    return Some(n);
                } else {
                    return Some(v);
                }
            }
            (Some(n), None) | (None, Some(n)) => Some(n),
            (None, None) => None,
        }
    }
}

impl Transformable for Rect {
    fn apply(mut self, transform: &Transform) -> Self {
        self.transform = self.transform.apply(transform);
        self
    }
}

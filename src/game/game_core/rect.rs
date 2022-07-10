use super::*;

#[derive(Clone)]
pub struct Rect {
    pub x1y1: Vec2,
    pub x2y2: Vec2,
}

impl Rect {
    pub fn corners(
        &self,
    ) -> (Vec2, Vec2, Vec2, Vec2) {
        (
            self.x1y1,
            Vec2::new(self.x1y1.x, self.x2y2.y),
            self.x2y2,
            Vec2::new(self.x2y2.x, self.x1y1.y),
        )
    }
    pub fn check_point_in_rect(
        &self,
        point: Vec2,
    ) -> bool {
        let x1 = self.x1y1.x;
        let y1 = self.x1y1.y;
        let x2 = self.x2y2.x;
        let y2 = self.x2y2.y;

        let x = point.x;
        let y = point.y;

        let in_x = x2 >= x && x1 <= x
            || x1 >= x && x2 <= x;
        let in_y = y2 >= y && y1 <= y
            || y1 >= y && y2 <= y;

        in_x && in_y
    }
    pub fn check_collision(
        static_rect: &Rect,
        moving_rect: &Rect,
        vel_vec: Vec2,
    ) -> Option<Vec2> {
        let (v1, v2, v3, v4) =
            moving_rect.corners();
        for i in [v1, v2, v3, v4] {
            if static_rect.check_point_in_rect(i)
            {
                //TODO lol hella fix this please
                return Some(-vel_vec);
            }
        }
        None
    }
}

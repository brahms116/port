use super::*;

#[derive(Clone, Debug)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
    pub transform: Transform,
}

impl Rect {
    pub fn two_point_flat(
        x1y1: Vec2,
        x2y2: Vec2,
    ) -> Self {
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
    pub fn corners(
        &self,
    ) -> (Vec2, Vec2, Vec2, Vec2) {
        let x = self.width / 2.0;
        let y = self.height / 2.0;

        (
            Vec2::new(-x, -y)
                .apply(&self.transform),
            Vec2::new(-x, y)
                .apply(&self.transform),
            Vec2::new(x, y)
                .apply(&self.transform),
            Vec2::new(x, -y)
                .apply(&self.transform),
        )
    }
    pub fn check_point_in_rect(
        &self,
        point: Vec2,
        debug: bool,
    ) -> bool {
        let diff =
            point - self.transform.position;

        if debug {
            web_sys::console::log_1(
                &format!("Diff, {:?}", diff)
                    .into(),
            );
            web_sys::console::log_1(
                &format!(
                    "self rotation, {:?}",
                    self.transform.rotation
                )
                .into(),
            );
        }

        let diff = diff.rotate_deg(
            360.0 - self.transform.rotation,
        );
        if debug {
            web_sys::console::log_1(
                &format!(
                    "Diff after rotate, {:?}",
                    diff
                )
                .into(),
            );
        }

        let x_diff = diff.x.abs();
        let y_diff = diff.y.abs();

        if debug {
            web_sys::console::log_1(
                &format!("x diff, {:?}", x_diff)
                    .into(),
            );

            web_sys::console::log_1(
                &format!("y diff, {:?}", y_diff)
                    .into(),
            );
        }

        self.width / 2.0 >= x_diff
            && self.height / 2.0 >= y_diff
    }
    pub fn check_collision(
        static_rect: &Rect,
        moving_rect: &Rect,
        vel_vec: Vec2,
    ) -> Option<Vec2> {
        //let (v1, v2, v3, v4) =
        //    moving_rect.corners();
        //for i in [v1, v2, v3, v4] {
        //    if static_rect.check_point_in_rect(i)
        //    {
        //        //TODO lol hella fix this please
        //        return Some(-vel_vec);
        //    }
        //}
        //

        let (v1, v2, v3, v4) =
            static_rect.corners();
        for i in [v1, v2, v3, v4] {
            if moving_rect
                .check_point_in_rect(i, false)
            {
                web_sys::console::log_1(
                    &"collided!".into(),
                );
                let mut checkee = -vel_vec;
                let mut offset = Vec2::default();
                let mut result: Option<Vec2> =
                    None;
                for _ in 0..3 {
                    let new_vec = checkee * 0.5;
                    let mut new_rect =
                        moving_rect.clone();
                    new_rect
                        .transform
                        .position +=
                        new_vec + offset;
                    web_sys::console::log_1(
                        &format!(
                            "moving rect {:?}",
                            moving_rect
                        )
                        .into(),
                    );
                    web_sys::console::log_1(
                        &format!(
                            "new rect {:?}",
                            new_rect
                        )
                        .into(),
                    );

                    if new_rect
                        .check_point_in_rect(
                            i, true,
                        )
                    {
                        offset += new_vec;
                        checkee = new_vec;
                    } else {
                        web_sys::console::log_1(
                            &format!(
                                "new vec {:?}",
                                new_vec
                            )
                            .into(),
                        );
                        web_sys::console::log_1(
                            &format!(
                                "offset  {:?}",
                                offset
                            )
                            .into(),
                        );
                        result = Some(
                            new_vec + offset,
                        );
                        checkee = new_vec;
                    }
                }
                let result = Some(
                    result.unwrap_or(-vel_vec),
                );
                return result;
            }
        }
        None
    }
}

impl Transformable for Rect {
    fn apply(
        mut self,
        transform: &Transform,
    ) -> Self {
        self.transform =
            self.transform.apply(transform);
        self
    }
}

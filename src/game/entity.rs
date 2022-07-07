use super::*;

pub fn basic_rect() -> (Position, Rotation, Surfaces<PSEVS>, MovementDelta, PSEVS) {
    fn cb(evs: &mut PSEVS, _size: &WindowSize) -> Vec<Surface> {
        let mut width = 32.0;
        let height = 24.0;
        let width_span = 800.0;

        match evs {
            PSEVS::WidthPOF(n) => {
                let val = n.poll();
                if val < 0.5 {
                    width += val * width_span
                } else {
                    width += width_span * (1.0 - val)
                }
            }
            _ => {}
        }

        let x = width / 2.0;
        let y = height / 2.0;

        vec![Surface {
            points: vec![
                Vec2::new(-x, -y),
                Vec2::new(-x, y),
                Vec2::new(x, y),
                Vec2::new(x, -y),
            ],
            color: RGBA::new(0, 0, 0, 1.0),
        }]
    }
    (
        Position(Vec2::default() - Vec2::x() * 200.0),
        Rotation(0.0),
        Surfaces(cb),
        MovementDelta(Vec2::new(1.5, 1.0)),
        PSEVS::WidthPOF(LinearAnimation::new(50)),
    )
}

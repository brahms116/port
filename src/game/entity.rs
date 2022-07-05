use super::game_components::*;
use super::game_core::*;

pub fn basic_rect() -> (
    Position,
    Rotation,
    Surfaces,
    MovementDelta,
    Opacity,
    OpacityDelta,
) {
    let surface = Surface {
        points: vec![
            Vec2::new(-16.0, -16.0),
            Vec2::new(-16.0, 16.0),
            Vec2::new(16.0, 16.0),
            Vec2::new(16.0, -16.0),
        ],
        color: RGBA::new(0, 0, 0, 1.0),
    };

    fn get_surfaces(n: SurfacesArgs) {
        let width = n.width.unwrap_or(32.0);
        let height = n.height.unwrap_or(32.0);
        let scale_x = n.scale_x.unwrap_or(1.0);
        let scale_y = n.scale_y.unwrap_or(1.0);
        let opt = n.opacity.unwrap_or(1.0);
    }

    (
        Position(Vec2::default()),
        Rotation(0.0),
        Surfaces(|_| vec![]),
        MovementDelta(Vec2::new(0.5, 0.1)),
        Opacity(0.0),
        OpacityDelta(0.01),
    )
}

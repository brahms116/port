use super::game_components::*;
use super::game_core::*;

pub fn basic_rect() -> (Position, Rotation, Surfaces, MovementDelta) {
    let surface = Surface {
        points: vec![
            Vec2::new(-16.0, -16.0),
            Vec2::new(-16.0, 16.0),
            Vec2::new(16.0, 16.0),
            Vec2::new(16.0, -16.0),
        ],
        color: String::from("#000000"),
    };
    (
        Position(Vec2::default()),
        Rotation(0.0),
        Surfaces(vec![surface]),
        MovementDelta(Vec2::default()),
    )
}

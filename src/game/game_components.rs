use super::*;

pub struct GameController();

pub struct Camera();

pub struct FrameCount(pub u32);

pub struct StageCount(pub u32);

pub struct Surfaces<T>(pub fn(&mut T, &WindowSize) -> Vec<Surface>);

pub struct Position(pub Vec2);

pub struct MovementDelta(pub Vec2);

pub struct Rotation(pub f64);

pub struct RotationDelta(pub f64);

/// Player Square Entity Visual State
pub enum PSEVS {
    Idle,
    WidthPOF(LinearAnimation),
}

impl Update for PSEVS {
    fn update(&mut self) {
        match self {
            Self::WidthPOF(n) => {
                if n.is_complete() {
                    n.reset();
                }
                n.advance_frame();
            }
            _ => {}
        }
    }
}

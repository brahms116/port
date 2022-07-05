use super::*;

pub struct GameController();

pub struct Camera();

pub struct FrameCount(pub u32);

pub struct StageCount(pub u32);

#[derive(Default)]
pub struct SurfacesArgs {
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub opacity: Option<f64>,
}

pub struct Surfaces(pub fn(SurfacesArgs) -> Vec<Surface>);

pub struct Position(pub Vec2);

pub struct MovementDelta(pub Vec2);

pub struct Rotation(pub f64);

pub struct RotationDelta(pub f64);

pub struct Width(pub f64);

pub struct WidthDelta(pub f64);

pub struct Height(pub f64);

pub struct HeightDelta(pub f64);

pub struct ScaleX(pub f64);

pub struct ScaleXDelta(pub f64);

pub struct ScaleY(pub f64);

pub struct ScaleYDelta(pub f64);

pub struct Opacity(pub f64);

pub struct OpacityDelta(pub f64);

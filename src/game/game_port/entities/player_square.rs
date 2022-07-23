use super::*;

const PLAYER_HEIGHT: f64 = 16.0;
const PLAYER_WIDTH: f64 = 16.0;
const PLAYER_COLOR: RGBA = RGBA {
    r: 0,
    g: 0,
    b: 0,
    a: 1.0,
};

pub fn create_player_square(
    transform: Transform,
) -> (
    Transform,
    Render,
    Height,
    Width,
    Rectangle,
    Player,
    Opacity,
    RenderOffset,
) {
    let surface = Surface {
        points: vec![],
        color: PLAYER_COLOR,
    };
    (
        transform,
        Render(vec![surface]),
        Height(PLAYER_HEIGHT),
        Width(PLAYER_WIDTH),
        Rectangle(),
        Player(),
        Opacity(1.0),
        RenderOffset(Vec2::default()),
    )
}

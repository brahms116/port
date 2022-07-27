use super::*;

pub const PLAYER_HEIGHT: f64 = 12.0;
pub const PLAYER_WIDTH: f64 = 12.0;
pub const PLAYER_MOVE_HEIGHT: f64 = 20.0;
pub const PLAYER_MOVE_WIDTH: f64 = 2.0;
pub const PLAYER_COLOR: RGBA = RGBA {
    r: 0,
    g: 0,
    b: 0,
    a: 1.0,
};

pub const PLAYER_VELOCITY: f64 = 5.0;
pub const PLAYER_ACCEL: f64 = 0.3;

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
    FadeAnimation,
    Motion,
    Movement,
    SquishAnimation,
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
        FadeAnimation::new(50).fade_in(),
        Motion::default(),
        Movement::new(TravelSettings::uniform_config(
            &TravelConfig {
                max_vel: PLAYER_VELOCITY,
                travel_accel: PLAYER_ACCEL,
            },
        )),
        SquishAnimation::new(
            SquishConfig {
                start_height: PLAYER_HEIGHT,
                start_width: PLAYER_WIDTH,
                squish_height: 2.0,
                squish_width: 100.0,
                finish_height: PLAYER_WIDTH,
                finish_width: PLAYER_WIDTH,
                should_anchor: true,
            },
            50,
        ),
    )
}

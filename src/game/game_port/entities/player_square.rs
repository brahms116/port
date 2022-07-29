use super::*;

pub const PLAYER_HEIGHT: f64 = 18.0;
pub const PLAYER_WIDTH: f64 = 12.0;
pub const PLAYER_SQUISH_HEIGHT: f64 = 2.0;
pub const PLAYER_SQUISH_WIDTH: f64 = 30.0;

pub const PLAYER_MOVE_HEIGHT: f64 = 20.0;
pub const PLAYER_MOVE_WIDTH: f64 = 8.0;
pub const PLAYER_COLOR: RGBA = RGBA {
    r: 221,
    g: 255,
    b: 218,
    a: 1.0,
};

pub const PLAYER_VELOCITY: f64 = 5.0;
pub const PLAYER_ACCEL: f64 = 0.3;

pub fn player_squish_setting_start() -> SquishConfig {
    SquishConfig {
        start_height: PLAYER_HEIGHT,
        start_width: PLAYER_WIDTH,
        squish_height: PLAYER_SQUISH_HEIGHT,
        squish_width: PLAYER_SQUISH_WIDTH,
        finish_height: PLAYER_MOVE_HEIGHT,
        finish_width: PLAYER_MOVE_WIDTH,
        should_anchor: true,
    }
}

pub fn player_squish_setting_stop() -> SquishConfig {
    SquishConfig {
        start_height: PLAYER_MOVE_HEIGHT,
        start_width: PLAYER_MOVE_WIDTH,
        squish_height: PLAYER_SQUISH_HEIGHT,
        squish_width: PLAYER_SQUISH_WIDTH,
        finish_height: PLAYER_HEIGHT,
        finish_width: PLAYER_WIDTH,
        should_anchor: true,
    }
}

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
    SquishMovement,
    Rotation,
) {
    let surface = Surface {
        points: vec![],
        color: PLAYER_COLOR,
    };

    let squish = SquishMovement::new();

    let mut travel_settings =
        TravelSettings::uniform_config(&TravelConfig {
            max_vel: PLAYER_VELOCITY,
            travel_accel: PLAYER_ACCEL,
        });
    travel_settings.back.max_vel = PLAYER_VELOCITY * 0.5;

    (
        transform,
        Render(vec![
            surface,
            // Surface {
            //     points: vec![
            //         Vec2::new(-4.0, -4.0),
            //         Vec2::new(0.0, 4.0),
            //         Vec2::new(4.0, -4.0),
            //     ],
            //     color: RGBA {
            //         r: 0,
            //         g: 255,
            //         b: 0,
            //         a: 1.0,
            //     },
            // },
        ]),
        Height(PLAYER_HEIGHT),
        Width(PLAYER_WIDTH),
        Rectangle(),
        Player(),
        Opacity(1.0),
        RenderOffset(Vec2::default()),
        FadeAnimation::new(50),
        Motion::default(),
        Movement::new(travel_settings),
        SquishAnimation::new(SquishConfig::default(), 15),
        squish,
        Rotation::new(RotationConfig::new(1.5)),
    )
}

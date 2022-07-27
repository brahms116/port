use std::collections::HashMap;

use super::*;

const PLAYER_HEIGHT: f64 = 12.0;
const PLAYER_WIDTH: f64 = 12.0;
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
    FadeAnimation,
    Motion,
    Movement,
    SquishAnimation,
) {
    let surface = Surface {
        points: vec![],
        color: PLAYER_COLOR,
    };

    let mut hash =
        HashMap::<MovementDirection, TravelConfig>::new();
    hash.insert(
        MovementDirection::Front,
        TravelConfig {
            max_vel: 5.0,
            travel_accel: 0.3,
        },
    );
    hash.insert(
        MovementDirection::Back,
        TravelConfig {
            max_vel: 5.0,
            travel_accel: 0.3,
        },
    );
    hash.insert(
        MovementDirection::Left,
        TravelConfig {
            max_vel: 5.0,
            travel_accel: 0.3,
        },
    );
    hash.insert(
        MovementDirection::Right,
        TravelConfig {
            max_vel: 5.0,
            travel_accel: 0.3,
        },
    );
    (
        transform,
        Render(vec![surface]),
        Height(PLAYER_HEIGHT),
        Width(PLAYER_WIDTH),
        Rectangle(),
        Player(),
        Opacity(1.0),
        RenderOffset(Vec2::default()),
        FadeAnimation {
            animation_type: FadeAnimationType::In,
            engine: LinearProgress::new(50),
            is_active: true,
        },
        Motion::default(),
        Movement {
            settings: TravelSettings(hash),
            direction: MovementDirection::Idle,
            applied_accel: 0.0,
        },
        SquishAnimation {
            direction: SquishDirection::Front,
            config: SquishConfig {
                start_height: PLAYER_HEIGHT,
                start_width: PLAYER_WIDTH,
                squish_height: 2.0,
                squish_width: 100.0,
                finish_height: PLAYER_WIDTH,
                finish_width: PLAYER_WIDTH,
                should_anchor: true,
            },
            engine: LinearProgress::new(30),
            is_active: true,
        },
    )
}

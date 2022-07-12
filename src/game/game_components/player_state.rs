use super::*;

pub struct PlayerState {
    pub state: PlayerStateKind,
    pub config: PlayerStateConfig,
}

pub struct PlayerStateConfig {
    pub max_travel_vel: f64,
    pub travel_accel: f64,
    pub squish_duration: u32,
}

impl Default for PlayerStateConfig {
    fn default() -> Self {
        Self {
            max_travel_vel: 10.0,
            travel_accel: 0.5,
            squish_duration: 15,
        }
    }
}

impl PlayerState {
    pub fn still() -> Self {
        Self {
            state: PlayerStateKind::Still,
            config: PlayerStateConfig::default(),
        }
    }
    pub fn post_motion(
        dir: PlayerDirection,
    ) -> Self {
        let config = PlayerStateConfig::default();
        Self {
            state: PlayerStateKind::PostMotion((
                Sequence::new(
                    config.squish_duration,
                ),
                dir,
            )),
            config,
        }
    }
    pub fn motion() -> Self {
        let config = PlayerStateConfig::default();
        Self {
            state: PlayerStateKind::Motion(
                Sequence::new(
                    config.squish_duration,
                ),
            ),
            config,
        }
    }
}

pub enum PlayerStateKind {
    Motion(Sequence),
    Still,
    Jump(Sequence),
    PostMotion((Sequence, PlayerDirection)),
}

pub enum PlayerDirection {
    Front,
    Back,
}

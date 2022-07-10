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
            max_travel_vel: 5.0,
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

    pub fn update(&mut self) {
        match &mut self.state {
            PlayerStateKind::PostMotion(n) => {
                n.0.advance_frame()
            }
            PlayerStateKind::Motion(n) => {
                n.advance_frame();
                if n.poll() > 0.3 {
                    n.advance_checkpoint();
                }
            }
            _ => {}
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

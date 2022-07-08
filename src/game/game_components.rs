use super::*;

pub struct GameController();

pub struct Camera();

pub struct FrameCount(pub u32);

pub struct StageCount(pub u32);

pub trait Transformable {
    fn apply(&mut self, transform: &Transform);
}

impl Transformable for Rect {
    // TODO Apply rotation here
    fn apply(&mut self, transform: &Transform) {
        self.x1y1 += transform.position;
        self.x2y2 += transform.position;
    }
}

#[derive(Default, Clone, Debug)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f64,
}

impl Transform {
    pub fn new(position: Vec2, rotation: f64) -> Self {
        Self { position, rotation }
    }
}

#[derive(Default, Debug)]
pub struct Motion {
    pub vel: Vec2,
    pub accel: Vec2,
    pub angular_vel: f64,
    pub angular_accel: f64,
}

pub struct BoxCollider {
    pub width: f64,
    pub height: f64,
    pub position: Vec2,
}

impl BoxCollider {
    pub fn new(width: f64, height: f64, center: Vec2) -> Self {
        Self {
            width,
            height,
            position: center,
        }
    }

    pub fn rect(&self) -> Rect {
        let x = self.width / 2.0;
        let y = self.height / 2.0;
        Rect {
            x2y2: Vec2::new(x, y),
            x1y1: Vec2::new(-x, -y),
        }
    }
}

pub struct StateColliderCb<State>(pub fn(&State) -> BoxCollider);

pub struct RenderStatic(pub Vec<Surface>);

pub struct StateRenderCb<State>(pub fn(&State, &WindowSize) -> Vec<Surface>);

pub struct StateMotionCb<State>(pub fn(&mut State, &mut Motion, &mut Transform));

// pub struct UpdateStateCb<State>(pub fn(&mut State));

pub enum PlayerDirection {
    Front,
    Back,
}

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
            travel_accel: 0.3,
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
    pub fn post_motion(dir: PlayerDirection) -> Self {
        let config = PlayerStateConfig::default();
        Self {
            state: PlayerStateKind::PostMotion((Sequence::new(config.squish_duration), dir)),
            config,
        }
    }
    pub fn motion() -> Self {
        let config = PlayerStateConfig::default();
        Self {
            state: PlayerStateKind::Motion(Sequence::new(config.squish_duration)),
            config,
        }
    }

    pub fn update(&mut self) {
        match &mut self.state {
            PlayerStateKind::PostMotion(n) => n.0.advance_frame(),
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

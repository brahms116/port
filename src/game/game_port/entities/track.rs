use super::*;

type S = TrackStateKind;

pub enum TrackStateKind {
    FadeIn(Sequence),
    FadeOut(Sequence),
    Still,
}

pub struct TrackStateConfig {
    pub fade_duration: u32,
}

impl Default for TrackStateConfig {
    fn default() -> Self {
        Self { fade_duration: 50 }
    }
}

pub struct TrackState {
    pub len: f64,
    pub state: TrackStateKind,
    pub config: TrackStateConfig,
}

impl TrackState {
    pub fn new(len: f64) -> Self {
        let config = TrackStateConfig::default();
        Self {
            len,
            state: TrackStateKind::Still,
            config,
        }
    }

    pub fn fade_in(&mut self) {
        self.state = TrackStateKind::FadeIn(
            Sequence::new(
                self.config.fade_duration,
            ),
        );
    }

    pub fn fade_out(&mut self) {
        self.state = TrackStateKind::FadeOut(
            Sequence::new(
                self.config.fade_duration,
            ),
        );
    }

    pub fn still(&mut self) {
        self.state = TrackStateKind::Still;
    }

    pub fn fade_in_chain(mut self) -> Self {
        self.fade_in();
        self
    }

    pub fn fade_out_chain(mut self) -> Self {
        self.fade_out();
        self
    }
}

pub fn track(
    transform: Transform,
    state: TrackState,
) -> (
    TrackState,
    Transform,
    StateRenderCb<TrackState>,
    UpdateStateCb<TrackState>,
) {
    (
        state,
        transform,
        StateRenderCb(render_cb),
        UpdateStateCb(update_state_cb),
    )
}

fn update_state_cb(state: &mut TrackState) {
    match &mut state.state {
        S::FadeOut(n) | S::FadeIn(n) => {
            if n.is_complete() {
                *state =
                    TrackState::new(state.len)
            } else {
                n.advance_frame();
            }
        }
        _ => {}
    }
}

fn render_cb(
    state: &TrackState,
    _size: &WindowSize,
) -> Vec<Surface> {
    let mut color = RGBA::new(230, 230, 230, 1.0);
    let len = state.len;
    let height = 4.0;
    let y = height / 2.0;

    let points: Vec<Vec2> = vec![
        Vec2::new(0.0, -y),
        Vec2::new(0.0, y),
        Vec2::new(len, y),
        Vec2::new(len, -y),
    ];

    match &state.state {
        S::FadeOut(n) | S::FadeIn(n) => {
            color.a = n.poll();
        }
        _ => {}
    }
    vec![Surface { points, color }]
}
